use crate::cli::args::{self, CliCommand};
use crate::cli::output;
use crate::commands::{self, CommandContext, CommandOutcome};

pub fn run() -> i32 {
    let parsed = args::parse(std::env::args().skip(1));

    let outcome = match parsed.command {
        CliCommand::Scan(options) => {
            match CommandContext::from_options(&options) {
                Ok(context) => commands::scan::run(options, &context),
                Err(error) => CommandOutcome::error("EnvSentinel", error),
            }
        }
        CliCommand::Diff(options) => {
            match CommandContext::from_options(&options) {
                Ok(context) => commands::diff::run(options, &context),
                Err(error) => CommandOutcome::error("EnvSentinel", error),
            }
        }
        CliCommand::Validate(options) => {
            match CommandContext::from_options(&options) {
                Ok(context) => commands::validate::run(options, &context),
                Err(error) => CommandOutcome::error("EnvSentinel", error),
            }
        }
        CliCommand::Init(options) => {
            match CommandContext::from_options(&options) {
                Ok(context) => commands::init::run(options, &context),
                Err(error) => CommandOutcome::error("EnvSentinel", error),
            }
        }
        CliCommand::Watch(options) => {
            match CommandContext::from_options(&options) {
                Ok(context) => return commands::watch::run(options, &context, parsed.output_format),
                Err(error) => CommandOutcome::error("EnvSentinel", error),
            }
        }
        CliCommand::Help => CommandOutcome::usage(
            "EnvSentinel commands: scan, diff, validate, init, watch. Use --json or --markdown for alternate output.",
        ),
    };

    output::render(&outcome, parsed.output_format);
    outcome.exit_code
}
