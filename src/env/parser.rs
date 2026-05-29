use std::path::Path;
use std::collections::HashSet;

use super::model::{EnvFile, EnvKey, ParseIssue, ParseIssueKind};

fn parse_value(raw_value: &str) -> (String, bool) {
    let value = raw_value.trim();

    if value.len() >= 2 {
        let bytes = value.as_bytes();
        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            return (value[1..value.len() - 1].to_string(), true);
        }
    }

    (value.to_string(), false)
}

fn needs_quotes(value: &str, already_quoted: bool) -> bool {
    !already_quoted && (value.contains(' ') || value.contains('#') || value.contains('\t'))
}

pub fn parse_file(path: impl AsRef<Path>, contents: &str) -> EnvFile {
    let mut keys = Vec::new();
    let mut issues = Vec::new();
    let mut seen = HashSet::new();

    for (index, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let stripped = trimmed.strip_prefix("export ").unwrap_or(trimmed).trim();

        let Some((name, value)) = stripped.split_once('=') else {
            issues.push(ParseIssue {
                kind: ParseIssueKind::MalformedLine,
                message: format!("Malformed env line: {}", line.trim()),
                key: None,
                line_number: Some(index + 1),
            });
            continue;
        };

        let name = name.trim();
        if name.is_empty() {
            issues.push(ParseIssue {
                kind: ParseIssueKind::MalformedLine,
                message: format!("Malformed env line: {}", line.trim()),
                key: None,
                line_number: Some(index + 1),
            });
            continue;
        }

        if !seen.insert(name.to_string()) {
            issues.push(ParseIssue {
                kind: ParseIssueKind::DuplicateKey,
                message: format!("Duplicate key: {}", name),
                key: Some(name.to_string()),
                line_number: Some(index + 1),
            });
            continue;
        }

        let (parsed_value, was_quoted) = parse_value(value);

        if parsed_value.is_empty() {
            issues.push(ParseIssue {
                kind: ParseIssueKind::EmptyValue,
                message: format!("Empty value for key: {}", name),
                key: Some(name.to_string()),
                line_number: Some(index + 1),
            });
        }

        if needs_quotes(value, was_quoted) {
            issues.push(ParseIssue {
                kind: ParseIssueKind::MissingQuotes,
                message: format!("Value for {} should be quoted", name),
                key: Some(name.to_string()),
                line_number: Some(index + 1),
            });
        }

        keys.push(EnvKey {
            name: name.to_string(),
            value: Some(parsed_value),
            source: Some(path.as_ref().to_path_buf()),
            line_number: Some(index + 1),
        });
    }

    EnvFile {
        path: path.as_ref().to_path_buf(),
        keys,
        issues,
    }
}
