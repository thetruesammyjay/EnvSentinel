use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defaults {
    pub ignore_directories: Vec<PathBuf>,
    pub strict: bool,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            ignore_directories: vec![PathBuf::from("node_modules"), PathBuf::from("target"), PathBuf::from("dist")],
            strict: false,
        }
    }
}
