use std::path::PathBuf;

use crate::cli::args::CommandOptions;
use crate::commands::{self, CommandContext, CommandOutcome};
use crate::env::{scanner, validate};

fn resolve_validation_targets(
    options: &CommandOptions,
    context: &CommandContext,
) -> Vec<PathBuf> {
    if !options.targets.is_empty() {
        return options
            .targets
            .iter()
            .map(|path| commands::resolve_path(&context.root, path))
            .collect();
    }

    if !context.config.defaults.target_files.is_empty() {
        return context
            .config
            .defaults
            .target_files
            .iter()
            .map(|path| commands::resolve_path(&context.root, path))
            .collect();
    }

    scanner::discover_candidates(&context.root, &context.config.defaults.ignore_directories)
}

pub fn run(options: CommandOptions, context: &CommandContext) -> CommandOutcome {
    let targets = resolve_validation_targets(&options, context);

    if targets.is_empty() {
        return CommandOutcome::new(
            2,
            format!(
                "No env files found under {}. Provide explicit targets or add .env files.",
                context.root.display()
            ),
        );
    }

    let mut lines = vec![format!("Root: {}", context.root.display())];
    let mut failed = false;

    for path in targets {
        lines.push(String::new());
        lines.push(format!("File: {}", path.display()));

        match commands::load_env_file(&path) {
            Ok(file) => {
                let result = validate::validate(&file);

                if result.errors.is_empty() && result.warnings.is_empty() {
                    lines.push(String::from("OK"));
                } else {
                    if !result.errors.is_empty() {
                        failed = true;
                        lines.push(format!("Errors: {}", commands::format_list(&result.errors)));
                    }
                    if !result.warnings.is_empty() {
                        lines.push(format!("Warnings: {}", commands::format_list(&result.warnings)));
                    }
                }
            }
            Err(error) => {
                failed = true;
                lines.push(format!("Error: {}", error));
            }
        }
    }

    CommandOutcome::new(if failed { 1 } else { 0 }, lines.join("\n"))
}
