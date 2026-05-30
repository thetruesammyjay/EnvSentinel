use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::args::CommandOptions;
use crate::config::file::{ConfigError, ConfigFile};
use crate::env::parser::parse_file;
use crate::env::model::EnvFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    Success,
    Findings,
    Error,
    Usage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportSection {
    pub heading: String,
    pub lines: Vec<String>,
}

impl ReportSection {
    pub fn new(heading: impl Into<String>, lines: Vec<String>) -> Self {
        Self {
            heading: heading.into(),
            lines,
        }
    }
}

pub mod diff;
pub mod init;
pub mod scan;
pub mod validate;
pub mod watch;

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub root: PathBuf,
    pub config_path: PathBuf,
    pub config: ConfigFile,
}

impl CommandContext {
    pub fn from_options(options: &CommandOptions) -> Result<Self, String> {
        let root = resolve_root(options);
        let config_path = options
            .config
            .clone()
            .map(|path| resolve_path(&root, &path))
            .unwrap_or_else(|| root.join("config").join("envsentinel.toml"));
        let config = ConfigFile::load(&config_path).map_err(|error: ConfigError| error.to_string())?;

        Ok(Self {
            root,
            config_path,
            config,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    pub exit_code: i32,
    pub kind: OutcomeKind,
    pub title: String,
    pub summary: String,
    pub sections: Vec<ReportSection>,
    pub message: String,
}

impl CommandOutcome {
    pub fn new(exit_code: i32, message: impl Into<String>) -> Self {
        let summary = message.into();
        let kind = match exit_code {
            0 => OutcomeKind::Success,
            1 => OutcomeKind::Findings,
            2 => OutcomeKind::Error,
            _ => OutcomeKind::Error,
        };

        Self {
            exit_code,
            kind,
            title: String::from("EnvSentinel"),
            summary: summary.clone(),
            sections: Vec::new(),
            message: summary,
        }
    }

    pub fn success(title: impl Into<String>, summary: impl Into<String>) -> Self {
        let title = title.into();
        let summary = summary.into();

        Self {
            exit_code: 0,
            kind: OutcomeKind::Success,
            title,
            summary: summary.clone(),
            sections: Vec::new(),
            message: summary,
        }
    }

    pub fn findings(title: impl Into<String>, summary: impl Into<String>) -> Self {
        let title = title.into();
        let summary = summary.into();

        Self {
            exit_code: 1,
            kind: OutcomeKind::Findings,
            title,
            summary: summary.clone(),
            sections: Vec::new(),
            message: summary,
        }
    }

    pub fn error(title: impl Into<String>, summary: impl Into<String>) -> Self {
        let title = title.into();
        let summary = summary.into();

        Self {
            exit_code: 2,
            kind: OutcomeKind::Error,
            title,
            summary: summary.clone(),
            sections: Vec::new(),
            message: summary,
        }
    }

    pub fn usage(summary: impl Into<String>) -> Self {
        let summary = summary.into();

        Self {
            exit_code: 2,
            kind: OutcomeKind::Usage,
            title: String::from("Usage"),
            summary: summary.clone(),
            sections: Vec::new(),
            message: summary,
        }
    }

    pub fn with_section(mut self, heading: impl Into<String>, lines: Vec<String>) -> Self {
        self.sections.push(ReportSection::new(heading, lines));
        self
    }

    pub fn with_sections(mut self, sections: Vec<ReportSection>) -> Self {
        self.sections = sections;
        self
    }

    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}

pub fn section(heading: impl Into<String>, lines: Vec<String>) -> ReportSection {
    ReportSection::new(heading, lines)
}

pub fn load_env_file(path: impl AsRef<Path>) -> Result<EnvFile, String> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("{}: {}", path.display(), error))?;
    Ok(parse_file(path, &contents))
}

pub fn resolve_root(options: &CommandOptions) -> PathBuf {
    match options.root.clone() {
        Some(path) if path.is_absolute() => path,
        Some(path) => std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path),
        None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

pub fn resolve_path(root: &Path, value: &PathBuf) -> PathBuf {
    if value.is_absolute() {
        value.clone()
    } else {
        root.join(value)
    }
}

pub fn format_list(items: &[String]) -> String {
    if items.is_empty() {
        String::from("none")
    } else {
        items.join(", ")
    }
}

pub fn format_paths(items: &[PathBuf]) -> String {
    if items.is_empty() {
        String::from("none")
    } else {
        items
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub fn preferred_template_name(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".env.example" | ".env.sample" | ".env.template")
    )
}
