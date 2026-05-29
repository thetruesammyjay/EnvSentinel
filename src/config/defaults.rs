use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defaults {
    pub ignore_directories: Vec<PathBuf>,
    pub template_file: Option<PathBuf>,
    pub target_files: Vec<PathBuf>,
    pub strict: bool,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            ignore_directories: vec![
                PathBuf::from("node_modules"),
                PathBuf::from("target"),
                PathBuf::from("dist"),
            ],
            template_file: None,
            target_files: Vec::new(),
            strict: false,
        }
    }
}
