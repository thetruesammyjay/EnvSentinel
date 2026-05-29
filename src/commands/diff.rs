use std::path::PathBuf;

use crate::cli::args::CommandOptions;
use crate::commands::{self, CommandContext, CommandOutcome};
use crate::env::compare;

fn resolve_template_path(
    options: &CommandOptions,
    context: &CommandContext,
    discovered: &[PathBuf],
) -> Option<PathBuf> {
    if let Some(template) = options.template.as_ref() {
        return Some(commands::resolve_path(&context.root, template));
    }

    if let Some(template) = context.config.defaults.template_file.as_ref() {
        return Some(commands::resolve_path(&context.root, template));
    }

    discovered
        .iter()
        .find(|path| commands::preferred_template_name(path))
        .cloned()
}

fn resolve_target_paths(
    options: &CommandOptions,
    context: &CommandContext,
    discovered: &[PathBuf],
    template_path: Option<&PathBuf>,
) -> Vec<PathBuf> {
    if !options.targets.is_empty() {
        return options
            .targets
            .iter()
            .map(|path| commands::resolve_path(&context.root, path))
            .collect();
    }

    let mut targets = if !context.config.defaults.target_files.is_empty() {
        context
            .config
            .defaults
            .target_files
            .iter()
            .map(|path| commands::resolve_path(&context.root, path))
            .collect()
    } else {
        discovered.to_vec()
    };

    if let Some(template_path) = template_path {
        targets.retain(|path| path != template_path);
    }

    targets
}

pub fn run(options: CommandOptions, context: &CommandContext) -> CommandOutcome {
    let discovered = crate::env::scanner::discover_candidates(
        &context.root,
        &context.config.defaults.ignore_directories,
    );
    let template_path = resolve_template_path(&options, context, &discovered);
    let target_paths = resolve_target_paths(&options, context, &discovered, template_path.as_ref());

    let Some(template_path) = template_path else {
        return CommandOutcome::new(
            2,
            format!(
                "No template env file found under {}. Provide --template or create .env.example.",
                context.root.display()
            ),
        );
    };

    if target_paths.is_empty() {
        return CommandOutcome::new(
            2,
            format!(
                "No target env files found for template {}.",
                template_path.display()
            ),
        );
    }

    let template_file = match commands::load_env_file(&template_path) {
        Ok(file) => file,
        Err(error) => return CommandOutcome::new(1, error),
    };

    let strict = options.strict || context.config.defaults.strict;
    let mut lines = vec![format!("Template: {}", template_path.display())];
    let mut failed = false;

    for target_path in target_paths {
        lines.push(String::new());
        lines.push(format!("Target: {}", target_path.display()));

        match commands::load_env_file(&target_path) {
            Ok(target_file) => {
                let comparison = compare::compare(&template_file, &target_file);

                if comparison.missing_keys.is_empty()
                    && comparison.extra_keys.is_empty()
                    && comparison.empty_values.is_empty()
                {
                    lines.push(String::from("No drift detected."));
                } else {
                    if !comparison.missing_keys.is_empty() {
                        failed = true;
                        lines.push(format!("Missing: {}", commands::format_list(&comparison.missing_keys)));
                    }
                    if !comparison.extra_keys.is_empty() {
                        if strict {
                            failed = true;
                        }
                        lines.push(format!("Extra: {}", commands::format_list(&comparison.extra_keys)));
                    }
                    if !comparison.empty_values.is_empty() {
                        failed = true;
                        lines.push(format!("Empty values: {}", commands::format_list(&comparison.empty_values)));
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
