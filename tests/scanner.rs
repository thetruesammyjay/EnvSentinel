use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use envsentinel::env::scanner::discover_candidates;

fn unique_temp_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("envsentinel-scanner-{suffix}"))
}

#[test]
fn discovers_common_env_files_and_skips_ignored_directories() {
    let root = unique_temp_dir();
    let ignored_dir = root.join("node_modules");
    let kept_dir = root.join("config");

    fs::create_dir_all(&ignored_dir).expect("create ignored dir");
    fs::create_dir_all(&kept_dir).expect("create kept dir");

    fs::write(root.join(".env"), "API_KEY=1").expect("write .env");
    fs::write(root.join(".env.example"), "API_KEY=").expect("write .env.example");
    fs::write(kept_dir.join(".env.local"), "DEBUG=true").expect("write .env.local");
    fs::write(ignored_dir.join(".env"), "SHOULD_NOT_BE_FOUND=1").expect("write ignored .env");

    let candidates = discover_candidates(&root, &[PathBuf::from("node_modules"), PathBuf::from("target"), PathBuf::from("dist")]);

    assert!(candidates.contains(&root.join(".env")));
    assert!(candidates.contains(&root.join(".env.example")));
    assert!(candidates.contains(&kept_dir.join(".env.local")));
    assert!(!candidates.contains(&ignored_dir.join(".env")));

    fs::remove_dir_all(&root).expect("cleanup temp tree");
}