use std::fs;
use std::path::PathBuf;

use crate::cli::args::CommandOptions;
use crate::commands::{self, CommandContext, CommandOutcome};
use crate::env::parser::parse_file;

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
                    "No source .env file found under {}. Provide one target file to bootstrap from.",
                    context.root.display()
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

    let source_contents = match fs::read_to_string(&source_path) {
        Ok(contents) => contents,
        Err(error) => {
            return CommandOutcome::new(
                1,
                format!("{}: {}", source_path.display(), error),
            )
        }
    };

    let source_file = parse_file(&source_path, &source_contents);
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

    if let Err(error) = fs::write(&target_path, lines.join("\n")) {
        return CommandOutcome::new(
            1,
            format!("{}: {}", target_path.display(), error),
        );
    }

    CommandOutcome::new(
        0,
        format!(
            "Generated {} from {} using {} key(s).",
            target_path.display(),
            source_path.display(),
            lines.len()
        ),
    )
}
