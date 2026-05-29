use crate::commands::CommandOutcome;

pub fn render(outcome: &CommandOutcome) -> String {
    format!(
        "{{\"message\":\"{}\",\"exit_code\":{}}}",
        outcome.message.replace('"', "\\\""),
        outcome.exit_code
    )
}
