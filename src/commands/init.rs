use std::fs;
use std::path::PathBuf;

use crate::cli::args::CommandOptions;
use crate::commands::{self, CommandContext, CommandOutcome};
use crate::env::validate;

fn resolve_source_path(options: &CommandOptions, context: &CommandContext) -> Option<PathBuf> {
    if let Some(first_target) = options.targets.first() {
        return Some(commands::resolve_path(&context.root, first_target));
    }

    let default_source = context.root.join(".env");
    if default_source.exists() {
        return Some(default_source);
    }

    let discovered = crate::env::scanner::discover_candidates(
        &context.root,
        &context.config.defaults.ignore_directories,
    );

    discovered.into_iter().find(|path| {
        matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some(".env")
        )
    })
}

pub fn run(options: CommandOptions, context: &CommandContext) -> CommandOutcome {
    let source_path = match resolve_source_path(&options, context) {
        Some(path) => path,
        None => {
            return CommandOutcome::new(
                2,
                format!(
                    "No source .env file found at {}. Create one before running init.",
                    context.root.join(".env").display()
                ),
            )
        }
    };

    let target_path = options
        .template
        .as_ref()
        .map(|path| commands::resolve_path(&context.root, path))
        .or_else(|| context.config.defaults.template_file.as_ref().map(|path| commands::resolve_path(&context.root, path)))
        .unwrap_or_else(|| context.root.join(".env.example"));

    if target_path.exists() {
        return CommandOutcome::new(
            1,
            format!("{} already exists; refusing to overwrite.", target_path.display()),
        );
    }

    let source_file = match commands::load_env_file(&source_path) {
        Ok(file) => file,
        Err(error) => return CommandOutcome::new(1, error),
    };

    let validation = validate::validate(&source_file);
    if !validation.errors.is_empty() {
        return CommandOutcome::new(
            1,
            format!(
                "{} has validation errors: {}",
                source_path.display(),
                commands::format_list(&validation.errors)
            ),
        );
    }

    let mut lines = Vec::new();

    for key in source_file.keys {
        lines.push(format!("{}=", key.name));
    }

    if let Some(parent) = target_path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            return CommandOutcome::new(
                1,
                format!("{}: {}", parent.display(), error),
            );
        }
    }

    let output = if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    };

    if let Err(error) = fs::write(&target_path, output) {
        return CommandOutcome::new(
            1,
            format!("{}: {}", target_path.display(), error),
        );
    }

    let mut message = format!(
        "Generated {} from {} using {} key(s).",
        target_path.display(),
        source_path.display(),
        lines.len()
    );

    if !validation.warnings.is_empty() {
        message.push_str(" Warnings: ");
        message.push_str(&commands::format_list(&validation.warnings));
    }

    CommandOutcome::new(
        0,
        message,
    )
}
