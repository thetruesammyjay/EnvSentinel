use envsentinel::env::model::ParseIssueKind;
use envsentinel::env::parser::parse_file;

#[test]
fn parses_keys_while_ignoring_comments_and_blank_lines() {
    let contents = r#"
# comment

API_KEY=abc123
export DEBUG=true
NAME="Env Sentinel"
"#;

    let file = parse_file("/tmp/.env", contents);

    assert_eq!(file.key_names(), vec!["API_KEY", "DEBUG", "NAME"]);
    assert_eq!(file.value_for("API_KEY"), Some("abc123"));
    assert_eq!(file.value_for("DEBUG"), Some("true"));
    assert_eq!(file.value_for("NAME"), Some("Env Sentinel"));
    assert!(file.issues.is_empty());
}

#[test]
fn records_duplicate_malformed_empty_and_missing_quotes_issues() {
    let contents = r#"
API_KEY=abc 123
API_KEY=second
MISSING_LINE
EMPTY=
QUOTED="value with spaces"
"#;

    let file = parse_file("/tmp/.env", contents);

    let kinds: Vec<ParseIssueKind> = file.issues.iter().map(|issue| issue.kind.clone()).collect();

    assert!(kinds.contains(&ParseIssueKind::MissingQuotes));
    assert!(kinds.contains(&ParseIssueKind::DuplicateKey));
    assert!(kinds.contains(&ParseIssueKind::MalformedLine));
    assert!(kinds.contains(&ParseIssueKind::EmptyValue));
    assert_eq!(file.value_for("EMPTY"), Some(""));
}