use std::path::PathBuf;

use envsentinel::env::compare::compare;
use envsentinel::env::model::{EnvFile, EnvKey, ParseIssue};

fn env_file(path: &str, entries: &[(&str, Option<&str>)]) -> EnvFile {
    EnvFile {
        path: PathBuf::from(path),
        keys: entries
            .iter()
            .enumerate()
            .map(|(index, (name, value))| EnvKey {
                name: (*name).to_string(),
                value: value.map(|value| value.to_string()),
                source: Some(PathBuf::from(path)),
                line_number: Some(index + 1),
            })
            .collect(),
        issues: Vec::<ParseIssue>::new(),
    }
}

#[test]
fn compares_template_and_target_keys() {
    let template = env_file(
        "/tmp/.env.example",
        &[("API_KEY", Some("")), ("DATABASE_URL", Some("")), ("DEBUG", Some(""))],
    );
    let target = env_file(
        "/tmp/.env",
        &[("API_KEY", Some("secret")), ("DEBUG", Some("")), ("EXTRA", Some("value"))],
    );

    let result = compare(&template, &target);

    assert_eq!(result.missing_keys, vec!["DATABASE_URL"]);
    assert_eq!(result.extra_keys, vec!["EXTRA"]);
    assert_eq!(result.empty_values, vec!["DEBUG"]);
}