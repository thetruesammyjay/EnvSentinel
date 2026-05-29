use std::fs;
use std::path::{Path, PathBuf};

use crate::fs::ignore;

fn is_env_candidate(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(name) if name == ".env" || name.starts_with(".env.") || name == ".env.example" || name == ".env.sample" || name == ".env.template"
    )
}

fn walk_directory(current: &Path, ignore_directories: &[PathBuf], output: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if ignore::should_ignore(&path, ignore_directories) {
            continue;
        }

        if path.is_dir() {
            walk_directory(&path, ignore_directories, output);
            continue;
        }

        if is_env_candidate(&path) {
            output.push(path.clone());
        }
    }
}

pub fn discover_candidates(root: impl Into<PathBuf>, ignore_directories: &[PathBuf]) -> Vec<PathBuf> {
    let root = root.into();
    let mut output = Vec::new();

    walk_directory(&root, ignore_directories, &mut output);
    output.sort();
    output.dedup();
    output
}
