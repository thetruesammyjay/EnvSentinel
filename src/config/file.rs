use std::fs;
use std::path::Path;

use super::defaults::Defaults;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFile {
    pub path: std::path::PathBuf,
    pub defaults: Defaults,
}

impl ConfigFile {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let mut defaults = Defaults::default();

        if let Ok(contents) = fs::read_to_string(&path) {
            apply_config(&contents, &mut defaults);
        }

        Self { path, defaults }
    }
}

fn apply_config(contents: &str, defaults: &mut Defaults) {
    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }

        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };

        let key = key.trim();
        let value = value.trim();

        match key {
            "strict" => {
                defaults.strict = value.eq_ignore_ascii_case("true");
            }
            "template" => {
                defaults.template_file = parse_string(value).map(std::path::PathBuf::from);
            }
            "targets" => {
                defaults.target_files = parse_string_list(value)
                    .into_iter()
                    .map(std::path::PathBuf::from)
                    .collect();
            }
            "ignore_directories" => {
                defaults.ignore_directories = parse_string_list(value)
                    .into_iter()
                    .map(std::path::PathBuf::from)
                    .collect();
            }
            _ => {}
        }
    }
}

fn parse_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        Some(trimmed[1..trimmed.len() - 1].to_string())
    } else if !trimmed.is_empty() {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn parse_string_list(value: &str) -> Vec<String> {
    let trimmed = value.trim();
    let inner = trimmed
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(trimmed);

    inner
        .split(',')
        .filter_map(parse_string)
        .collect()
}
