use crate::commands::CommandOutcome;

pub fn render(outcome: &CommandOutcome) -> String {
    format!("# EnvSentinel\n\n{}", outcome.message)
}
