use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::cli::args::CommandOptions;
use crate::cli::output::OutputFormat;
use crate::cli::output;
use crate::commands::{CommandContext, CommandOutcome};

const WATCH_INTERVAL: Duration = Duration::from_secs(2);

pub fn startup_outcome(context: &CommandContext) -> CommandOutcome {
    CommandOutcome::new(
        0,
        format!(
            "Watching {} for env file changes every {}s. Press Ctrl+C to stop.",
            context.root.display(),
            WATCH_INTERVAL.as_secs()
        ),
    )
}

pub fn build_cycle_outcome(options: &CommandOptions, context: &CommandContext) -> CommandOutcome {
    let scan_outcome = crate::commands::scan::run(options.clone(), context);
    let validate_outcome = crate::commands::validate::run(options.clone(), context);

    let exit_code = if scan_outcome.exit_code == 2 || validate_outcome.exit_code == 2 {
        2
    } else if scan_outcome.exit_code == 1 || validate_outcome.exit_code == 1 {
        1
    } else {
        0
    };

    let message = format!(
        "Watch cycle for {}\n\n[scan]\n{}\n\n[validate]\n{}",
        context.root.display(),
        scan_outcome.message,
        validate_outcome.message
    );

    CommandOutcome::new(exit_code, message)
}

fn render_cycle(outcome: &CommandOutcome, format: OutputFormat) {
    output::render(outcome, format);
    let _ = io::stdout().flush();
}

pub fn run(options: CommandOptions, context: &CommandContext, format: OutputFormat) -> i32 {
    let startup = startup_outcome(context);
    output::render(&startup, format);
    let _ = io::stdout().flush();

    let mut last_message = String::new();

    loop {
        let cycle = build_cycle_outcome(&options, context);

        if cycle.message != last_message {
            render_cycle(&cycle, format);
            last_message = cycle.message.clone();
        }

        thread::sleep(WATCH_INTERVAL);
    }
}
