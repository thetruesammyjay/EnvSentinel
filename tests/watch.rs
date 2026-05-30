use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use envsentinel::cli::args::CommandOptions;
use envsentinel::commands::{watch, CommandContext};
use envsentinel::report::{json, markdown, text};

fn unique_temp_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("envsentinel-watch-{suffix}"))
}

#[test]
fn watch_cycle_combines_scan_and_validate_results() {
    let root = unique_temp_dir();

    fs::create_dir_all(&root).expect("create temp root");
    fs::write(root.join(".env.example"), "API_KEY=\nDATABASE_URL=\n").expect("write template");
    fs::write(root.join(".env"), "API_KEY=secret\nEXTRA=value\n").expect("write target");

    let options = CommandOptions {
        root: Some(root.clone()),
        config: None,
        template: Some(PathBuf::from(".env.example")),
        targets: vec![PathBuf::from(".env")],
        strict: false,
    };

    let context = CommandContext::from_options(&options).expect("load context");
    let cycle = watch::build_cycle_outcome(&options, &context);

    assert_eq!(cycle.exit_code, 1);
    assert!(cycle.message.contains("[scan]"));
    assert!(cycle.message.contains("[validate]"));
    assert!(cycle.message.contains("Missing: DATABASE_URL"));
    assert!(cycle.message.contains("Extra: EXTRA"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn watch_startup_banner_renders_through_shared_output_formats() {
    let root = unique_temp_dir();

    fs::create_dir_all(&root).expect("create temp root");

    let options = CommandOptions {
        root: Some(root.clone()),
        config: None,
        template: None,
        targets: Vec::new(),
        strict: false,
    };

    let context = CommandContext::from_options(&options).expect("load context");
    let startup = watch::startup_outcome(&context);

    let text_output = text::render(&startup);
    let markdown_output = markdown::render(&startup);
    let json_output = json::render(&startup);

    assert!(text_output.contains("Watching"));
    assert!(text_output.contains("Press Ctrl+C to stop."));
    assert!(markdown_output.starts_with("# EnvSentinel"));
    assert!(markdown_output.contains("Watching"));
    assert!(json_output.contains("\"message\":"));
    assert!(json_output.contains("Watching"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}