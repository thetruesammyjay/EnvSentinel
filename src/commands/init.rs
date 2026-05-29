use crate::cli::args::CommandOptions;
use crate::commands::CommandOutcome;

pub fn run(_options: CommandOptions) -> CommandOutcome {
    CommandOutcome::new(0, "init is scaffolded and ready for implementation")
}
