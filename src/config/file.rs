use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use super::defaults::Defaults;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub path: PathBuf,
    pub issues: Vec<String>,
}

impl ConfigError {
    fn new(path: PathBuf, issues: Vec<String>) -> Self {
        Self { path, issues }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.issues.join("; "))
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub defaults: Defaults,
}

impl ConfigFile {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Ok(Self {
                path,
                defaults: Defaults::default(),
            });
        }

        let contents = fs::read_to_string(&path)
            .map_err(|error| ConfigError::new(path.clone(), vec![format!("Failed to read config: {}", error)]))?;

        Self::from_str(path, &contents)
    }

    pub fn from_str(path: impl AsRef<Path>, contents: &str) -> Result<Self, ConfigError> {
        let path = path.as_ref().to_path_buf();
        let value: toml::Value = contents
            .parse()
            .map_err(|error| ConfigError::new(path.clone(), vec![format!("Invalid TOML: {}", error)]))?;

        let table = value.as_table().ok_or_else(|| {
            ConfigError::new(
                path.clone(),
                vec![String::from("Config root must be a TOML table")],
            )
        })?;

        let mut defaults = Defaults::default();
        let mut issues = Vec::new();

        for key in table.keys() {
            if key != "defaults" {
                issues.push(format!("Unknown top-level key `{}`", key));
            }
        }

        let Some(defaults_value) = table.get("defaults") else {
            issues.push(String::from("Missing [defaults] table"));
            return Err(ConfigError::new(path, issues));
        };

        let Some(defaults_table) = defaults_value.as_table() else {
            issues.push(String::from("`defaults` must be a table"));
            return Err(ConfigError::new(path, issues));
        };

        apply_defaults_table(defaults_table, &mut defaults, &mut issues);

        if issues.is_empty() {
            Ok(Self { path, defaults })
        } else {
            Err(ConfigError::new(path, issues))
        }
    }
}

fn apply_defaults_table(table: &toml::value::Table, defaults: &mut Defaults, issues: &mut Vec<String>) {
    for (key, value) in table {
        match key.as_str() {
            "strict" => {
                if let Some(flag) = value.as_bool() {
                    defaults.strict = flag;
                } else {
                    issues.push(format!(
                        "`defaults.strict` must be a boolean, got {}",
                        value_kind(value)
                    ));
                }
            }
            "template_file" => match value.as_str() {
                Some(text) if !text.trim().is_empty() => {
                    defaults.template_file = Some(PathBuf::from(text));
                }
                Some(_) => issues.push(String::from("`defaults.template_file` cannot be empty")),
                None => issues.push(format!(
                    "`defaults.template_file` must be a string, got {}",
                    value_kind(value)
                )),
            },
            "target_files" => {
                defaults.target_files = parse_path_list(value, "defaults.target_files", issues);
            }
            "ignore_directories" => {
                defaults.ignore_directories = parse_path_list(value, "defaults.ignore_directories", issues);
            }
            _ => issues.push(format!("Unknown key `defaults.{}`", key)),
        }
    }
}

fn parse_path_list(value: &toml::Value, field_name: &str, issues: &mut Vec<String>) -> Vec<PathBuf> {
    let Some(array) = value.as_array() else {
        issues.push(format!("`{}` must be an array of strings, got {}", field_name, value_kind(value)));
        return Vec::new();
    };

    let mut paths = Vec::new();

    for item in array {
        match item.as_str() {
            Some(text) if !text.trim().is_empty() => paths.push(PathBuf::from(text)),
            Some(_) => issues.push(format!("`{}` cannot contain empty strings", field_name)),
            None => issues.push(format!("`{}` must contain only strings, got {}", field_name, value_kind(item))),
        }
    }

    paths
}

fn value_kind(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "boolean",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}
