use std::collections::HashSet;

use super::model::EnvFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonResult {
    pub missing_keys: Vec<String>,
    pub extra_keys: Vec<String>,
    pub empty_values: Vec<String>,
}

pub fn compare(template: &EnvFile, target: &EnvFile) -> ComparisonResult {
    let template_keys: HashSet<&str> = template.keys.iter().map(|key| key.name.as_str()).collect();
    let target_keys: HashSet<&str> = target.keys.iter().map(|key| key.name.as_str()).collect();

    ComparisonResult {
        missing_keys: template
            .keys
            .iter()
            .filter(|key| !target_keys.contains(key.name.as_str()))
            .map(|key| key.name.clone())
            .collect(),
        extra_keys: target
            .keys
            .iter()
            .filter(|key| !template_keys.contains(key.name.as_str()))
            .map(|key| key.name.clone())
            .collect(),
        empty_values: target.empty_keys(),
    }
}
