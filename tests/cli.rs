use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("{prefix}-{suffix}"))
}

fn write_file(path: impl AsRef<std::path::Path>, contents: &str) {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }

    fs::write(path, contents).expect("write test fixture");
}

fn run_cli(args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_envsentinel"))
        .args(args)
        .output()
        .expect("run envsentinel binary");

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    (code, stdout)
}

#[test]
fn scan_reports_no_drift_and_exits_successfully() {
    let root = unique_temp_dir("envsentinel-scan");

    write_file(root.join(".env.example"), "API_KEY=\nDEBUG=\n");
    write_file(root.join(".env"), "API_KEY=secret\nDEBUG=true\n");
    write_file(root.join(".env.local"), "API_KEY=secret\nDEBUG=true\n");

    let (exit_code, stdout) = run_cli(&["scan", "--root", root.to_str().expect("temp path")]);

    assert_eq!(exit_code, 0);
    assert!(stdout.contains("Template:"));
    assert!(stdout.contains("Drift: none"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn validate_exits_with_ci_failure_on_malformed_env() {
    let root = unique_temp_dir("envsentinel-validate");

    write_file(root.join("bad.env"), "GOOD=value\nGOOD=second\nBROKEN_LINE\n");

    let (exit_code, stdout) = run_cli(&[
        "validate",
        "--root",
        root.to_str().expect("temp path"),
        "--target",
        "bad.env",
    ]);

    assert_eq!(exit_code, 1);
    assert!(stdout.contains("Errors:"));
    assert!(stdout.contains("Duplicate key: GOOD"));
    assert!(stdout.contains("Malformed env line: BROKEN_LINE"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn diff_reports_missing_and_extra_keys_with_failure_exit_code() {
    let root = unique_temp_dir("envsentinel-diff");

    write_file(root.join(".env.example"), "API_KEY=\nDATABASE_URL=\n");
    write_file(root.join(".env"), "API_KEY=secret\nEXTRA_KEY=value\n");

    let (exit_code, stdout) = run_cli(&[
        "diff",
        "--root",
        root.to_str().expect("temp path"),
        "--template",
        ".env.example",
        "--target",
        ".env",
    ]);

    assert_eq!(exit_code, 1);
    assert!(stdout.contains("Missing: DATABASE_URL"));
    assert!(stdout.contains("Extra: EXTRA_KEY"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn validate_can_render_json_output_for_ci_automation() {
    let root = unique_temp_dir("envsentinel-json");

    write_file(root.join("bad.env"), "GOOD=value\nGOOD=second\nBROKEN_LINE\n");

    let (exit_code, stdout) = run_cli(&[
        "validate",
        "--json",
        "--root",
        root.to_str().expect("temp path"),
        "--target",
        "bad.env",
    ]);

    let rendered = stdout.trim();

    assert_eq!(exit_code, 1);
    assert!(rendered.starts_with('{'));
    assert!(rendered.contains("\"exit_code\":1"));
    assert!(rendered.contains("Duplicate key: GOOD"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn scan_can_render_markdown_output() {
    let root = unique_temp_dir("envsentinel-markdown");

    write_file(root.join(".env.example"), "API_KEY=\nDEBUG=\n");
    write_file(root.join(".env"), "API_KEY=secret\nDEBUG=true\n");

    let (exit_code, stdout) = run_cli(&[
        "scan",
        "--markdown",
        "--root",
        root.to_str().expect("temp path"),
    ]);

    let rendered = stdout.trim();

    assert_eq!(exit_code, 0);
    assert!(rendered.starts_with("# EnvSentinel"));
    assert!(rendered.contains("Drift: none"));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}