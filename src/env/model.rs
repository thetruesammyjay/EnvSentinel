use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvKey {
    pub name: String,
    pub value: Option<String>,
    pub source: Option<PathBuf>,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvFile {
    pub path: PathBuf,
    pub keys: Vec<EnvKey>,
}
