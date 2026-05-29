use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseIssueKind {
    DuplicateKey,
    MalformedLine,
    MissingQuotes,
    EmptyValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIssue {
    pub kind: ParseIssueKind,
    pub message: String,
    pub key: Option<String>,
    pub line_number: Option<usize>,
}

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
    pub issues: Vec<ParseIssue>,
}

impl EnvFile {
    pub fn key_names(&self) -> Vec<String> {
        self.keys.iter().map(|key| key.name.clone()).collect()
    }

    pub fn value_for(&self, name: &str) -> Option<&str> {
        self.keys
            .iter()
            .find(|key| key.name == name)
            .and_then(|key| key.value.as_deref())
    }

    pub fn empty_keys(&self) -> Vec<String> {
        self.keys
            .iter()
            .filter(|key| key.value.as_deref().unwrap_or("").is_empty())
            .map(|key| key.name.clone())
            .collect()
    }
}
