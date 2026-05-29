use crate::commands::CommandOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

pub fn render(outcome: &CommandOutcome, format: OutputFormat) {
    match format {
        OutputFormat::Text => println!("{}", outcome.message),
        OutputFormat::Json => println!("{{\"message\":\"{}\",\"exit_code\":{}}}", outcome.message.replace('"', "\\\""), outcome.exit_code),
        OutputFormat::Markdown => println!("# EnvSentinel\n\n{}", outcome.message),
    }
}
