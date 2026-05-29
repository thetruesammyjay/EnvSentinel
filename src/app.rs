use crate::cli::args::{self, CliCommand};
use crate::cli::output;
use crate::commands::{self, CommandContext, CommandOutcome};

pub fn run() -> i32 {
    let parsed = args::parse(std::env::args().skip(1));

    let outcome = match parsed.command {
        CliCommand::Scan(options) => {
            let context = CommandContext::from_options(&options);
            commands::scan::run(options, &context)
        }
        CliCommand::Diff(options) => {
            let context = CommandContext::from_options(&options);
            commands::diff::run(options, &context)
        }
        CliCommand::Validate(options) => {
            let context = CommandContext::from_options(&options);
            commands::validate::run(options, &context)
        }
        CliCommand::Init(options) => {
            let context = CommandContext::from_options(&options);
            commands::init::run(options, &context)
        }
        CliCommand::Watch(options) => {
            let context = CommandContext::from_options(&options);
            commands::watch::run(options, &context)
        }
        CliCommand::Help => CommandOutcome::usage(
            "EnvSentinel commands: scan, diff, validate, init, watch. Use --json or --markdown for alternate output.",
        ),
    };

    output::render(&outcome, parsed.output_format);
    outcome.exit_code
}
