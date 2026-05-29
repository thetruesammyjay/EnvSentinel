use crate::cli::args::CommandOptions;
use crate::commands::CommandOutcome;

pub fn run(_options: CommandOptions) -> CommandOutcome {
    CommandOutcome::new(0, "diff is scaffolded and ready for implementation")
}
