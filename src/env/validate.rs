use super::model::EnvFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate(_file: &EnvFile) -> ValidationResult {
    ValidationResult {
        errors: Vec::new(),
        warnings: Vec::new(),
    }
}
