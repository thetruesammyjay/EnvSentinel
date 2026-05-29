use crate::cli::args::CommandOptions;
use crate::commands::{CommandContext, CommandOutcome};

pub fn run(_options: CommandOptions, context: &CommandContext) -> CommandOutcome {
    CommandOutcome::new(
        0,
        format!(
            "Watch mode is not implemented yet. Validation root: {}.",
            context.root.display()
        ),
    )
}
