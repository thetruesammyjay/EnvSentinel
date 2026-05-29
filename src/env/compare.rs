use super::model::EnvFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResult {
    pub missing_keys: Vec<String>,
    pub extra_keys: Vec<String>,
}

pub fn compare(_template: &EnvFile, _target: &EnvFile) -> ComparisonResult {
    ComparisonResult {
        missing_keys: Vec::new(),
        extra_keys: Vec::new(),
    }
}
