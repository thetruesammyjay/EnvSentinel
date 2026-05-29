use std::path::{Path, PathBuf};

use crate::cli::args::CommandOptions;
use crate::commands::{self, CommandContext, CommandOutcome};
use crate::env::{compare, scanner, validate};

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
    template_path: Option<&Path>,
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

    discovered
        .iter()
        .filter(|path| Some(path.as_path()) != template_path)
        .cloned()
        .collect()
}

pub fn run(options: CommandOptions, context: &CommandContext) -> CommandOutcome {
    let strict = options.strict || context.config.defaults.strict;
    let discovered = scanner::discover_candidates(&context.root, &context.config.defaults.ignore_directories);
    let template_path = resolve_template_path(&options, context, &discovered);
    let target_paths = resolve_target_paths(&options, context, &discovered, template_path.as_deref());

    let mut lines = vec![format!("Root: {}", context.root.display())];
    lines.push(format!("Config: {}", context.config_path.display()));
    lines.push(format!("Discovered env files: {}", discovered.len()));

    if let Some(path) = template_path.as_ref() {
        lines.push(format!("Template: {}", path.display()));
    } else {
        lines.push(String::from("Template: not found"));
    }

    if target_paths.is_empty() {
        lines.push(String::from("No target env files found."));
        return CommandOutcome::new(2, lines.join("\n"));
    }

    let template_file = template_path
        .as_ref()
        .and_then(|path| commands::load_env_file(path).ok());

    let mut failed = false;

    for target_path in target_paths {
        lines.push(String::new());
        lines.push(format!("File: {}", target_path.display()));

        match commands::load_env_file(&target_path) {
            Ok(target_file) => {
                let validation = validate::validate(&target_file);
                if !validation.errors.is_empty() {
                    failed = true;
                    lines.push(format!("Errors: {}", commands::format_list(&validation.errors)));
                }
                if !validation.warnings.is_empty() {
                    lines.push(format!("Warnings: {}", commands::format_list(&validation.warnings)));
                }

                if let Some(template_file) = template_file.as_ref() {
                    let comparison = compare::compare(template_file, &target_file);
                    if comparison.missing_keys.is_empty()
                        && comparison.extra_keys.is_empty()
                        && comparison.empty_values.is_empty()
                    {
                        lines.push(String::from("Drift: none"));
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
            }
            Err(error) => {
                failed = true;
                lines.push(format!("Error: {}", error));
            }
        }
    }

    CommandOutcome::new(if failed { 1 } else { 0 }, lines.join("\n"))
}
