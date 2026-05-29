use crate::commands::CommandOutcome;

fn escape_json(value: &str) -> String {
    let mut escaped = String::new();

    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(character),
        }
    }

    escaped
}

pub fn render(outcome: &CommandOutcome) -> String {
    format!(
        "{{\"message\":\"{}\",\"exit_code\":{}}}",
        escape_json(&outcome.message),
        outcome.exit_code
    )
}
