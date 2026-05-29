use std::path::PathBuf;

use super::output::OutputFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandName {
    Scan,
    Diff,
    Validate,
    Init,
    Watch,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedArgs {
    pub command: CliCommand,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    Scan(CommandOptions),
    Diff(CommandOptions),
    Validate(CommandOptions),
    Init(CommandOptions),
    Watch(CommandOptions),
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOptions {
    pub root: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub template: Option<PathBuf>,
    pub targets: Vec<PathBuf>,
    pub strict: bool,
}

impl Default for CommandOptions {
    fn default() -> Self {
        Self {
            root: None,
            config: None,
            template: None,
            targets: Vec::new(),
            strict: false,
        }
    }
}

pub fn parse<I>(args: I) -> ParsedArgs
where
    I: IntoIterator<Item = String>,
{
    let collected: Vec<String> = args.into_iter().collect();

    if collected.is_empty() {
        return ParsedArgs {
            command: CliCommand::Help,
            output_format: OutputFormat::Text,
        };
    }

    let mut output_format = OutputFormat::Text;
    let mut command_name = CommandName::Help;
    let mut options = CommandOptions::default();
    let mut index = 0;

    while index < collected.len() {
        let argument = &collected[index];
        match argument.as_str() {
            "scan" => command_name = CommandName::Scan,
            "diff" => command_name = CommandName::Diff,
            "validate" => command_name = CommandName::Validate,
            "init" => command_name = CommandName::Init,
            "watch" => command_name = CommandName::Watch,
            "--json" => output_format = OutputFormat::Json,
            "--markdown" => output_format = OutputFormat::Markdown,
            "--strict" => options.strict = true,
            "--root" => {
                if let Some(next) = collected.get(index + 1) {
                    options.root = Some(PathBuf::from(next));
                    index += 1;
                }
            }
            "--config" => {
                if let Some(next) = collected.get(index + 1) {
                    options.config = Some(PathBuf::from(next));
                    index += 1;
                }
            }
            "--template" => {
                if let Some(next) = collected.get(index + 1) {
                    options.template = Some(PathBuf::from(next));
                    index += 1;
                }
            }
            "--target" => {
                if let Some(next) = collected.get(index + 1) {
                    options.targets.push(PathBuf::from(next));
                    index += 1;
                }
            }
            _ => {
                if !argument.starts_with("--") && command_name != CommandName::Help {
                    options.targets.push(PathBuf::from(argument));
                }
            }
        }

        index += 1;
    }

    let command = match command_name {
        CommandName::Scan => CliCommand::Scan(options),
        CommandName::Diff => CliCommand::Diff(options),
        CommandName::Validate => CliCommand::Validate(options),
        CommandName::Init => CliCommand::Init(options),
        CommandName::Watch => CliCommand::Watch(options),
        CommandName::Help => CliCommand::Help,
    };

    ParsedArgs {
        command,
        output_format,
    }
}
