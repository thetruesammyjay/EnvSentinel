use crate::commands::CommandOutcome;
use crate::report::{json, markdown, text};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

pub fn render(outcome: &CommandOutcome, format: OutputFormat) {
    match format {
        OutputFormat::Text => println!("{}", text::render(outcome)),
        OutputFormat::Json => println!("{}", json::render(outcome)),
        OutputFormat::Markdown => println!("{}", markdown::render(outcome)),
    }
}
