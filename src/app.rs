use crate::cli::args::{self, CliCommand};
use crate::cli::output;
use crate::commands::{self, CommandOutcome};

pub fn run() -> i32 {
    let parsed = args::parse(std::env::args().skip(1));

    let outcome = match parsed.command {
        CliCommand::Scan(options) => commands::scan::run(options),
        CliCommand::Diff(options) => commands::diff::run(options),
        CliCommand::Validate(options) => commands::validate::run(options),
        CliCommand::Init(options) => commands::init::run(options),
        CliCommand::Watch(options) => commands::watch::run(options),
        CliCommand::Help => CommandOutcome::new(
            0,
            "EnvSentinel is scaffolded. Run a command like `scan`, `diff`, or `validate` once implemented.",
        ),
    };

    output::render(&outcome, parsed.output_format);
    outcome.exit_code
}
