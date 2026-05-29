use super::model::EnvFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate(file: &EnvFile) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for issue in &file.issues {
        match issue.kind {
            super::model::ParseIssueKind::MalformedLine => errors.push(issue.message.clone()),
            super::model::ParseIssueKind::DuplicateKey => errors.push(issue.message.clone()),
            super::model::ParseIssueKind::MissingQuotes => warnings.push(issue.message.clone()),
            super::model::ParseIssueKind::EmptyValue => warnings.push(issue.message.clone()),
        }
    }

    for key in file.empty_keys() {
        let message = format!("Empty value for key: {}", key);
        if !warnings.iter().any(|existing| existing == &message) {
            warnings.push(message);
        }
    }

    ValidationResult { errors, warnings }
}
