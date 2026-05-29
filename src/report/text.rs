use crate::commands::CommandOutcome;

pub fn render(outcome: &CommandOutcome) -> String {
    outcome.message.clone()
}
