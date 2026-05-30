use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use envsentinel::config::defaults::Defaults;
use envsentinel::config::file::ConfigFile;

fn unique_temp_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("envsentinel-config-{suffix}"))
}

#[test]
fn loads_config_from_defaults_table() {
    let root = unique_temp_dir();
    let config_path = root.join("config").join("envsentinel.toml");

    fs::create_dir_all(config_path.parent().expect("config parent")).expect("create config dir");
    fs::write(
        &config_path,
        r#"
[defaults]
strict = true
template_file = ".env.example"
target_files = [".env", ".env.local"]
ignore_directories = ["node_modules", "target", "dist"]
"#,
    )
    .expect("write config");

    let config = ConfigFile::load(&config_path).expect("load config");

    assert!(config.defaults.strict);
    assert_eq!(config.defaults.template_file, Some(PathBuf::from(".env.example")));
    assert_eq!(config.defaults.target_files, vec![PathBuf::from(".env"), PathBuf::from(".env.local")]);
    assert_eq!(config.defaults.ignore_directories, vec![PathBuf::from("node_modules"), PathBuf::from("target"), PathBuf::from("dist")]);

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn rejects_unknown_keys_and_invalid_types() {
    let root = unique_temp_dir();
    let config_path = root.join("config").join("envsentinel.toml");

    fs::create_dir_all(config_path.parent().expect("config parent")).expect("create config dir");
    fs::write(
        &config_path,
        r#"
[defaults]
strict = "yes"
template_file = 123
target_files = [".env", 42]
ignore_directories = [""]
extra_key = true

[other]
value = true
"#,
    )
    .expect("write config");

    let error = ConfigFile::load(&config_path).expect_err("config should fail validation");

    assert!(error.issues.iter().any(|issue| issue.contains("`defaults.strict` must be a boolean")));
    assert!(error.issues.iter().any(|issue| issue.contains("`defaults.template_file` must be a string")));
    assert!(error.issues.iter().any(|issue| issue.contains("`defaults.target_files` must contain only strings")));
    assert!(error.issues.iter().any(|issue| issue.contains("`defaults.ignore_directories` cannot contain empty strings")));
    assert!(error.issues.iter().any(|issue| issue.contains("Unknown key `defaults.extra_key`")));
    assert!(error.issues.iter().any(|issue| issue.contains("Unknown top-level key `other`")));

    fs::remove_dir_all(&root).expect("cleanup temp project");
}

#[test]
fn missing_config_file_falls_back_to_defaults() {
    let path = unique_temp_dir().join("config").join("envsentinel.toml");

    let config = ConfigFile::load(&path).expect("missing config should not fail");

    assert_eq!(config.defaults, Defaults::default());
}