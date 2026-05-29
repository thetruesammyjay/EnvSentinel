use std::path::Path;

use super::model::{EnvFile, EnvKey};

pub fn parse_file(path: impl AsRef<Path>, contents: &str) -> EnvFile {
    let mut keys = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((name, value)) = trimmed.split_once('=') {
            keys.push(EnvKey {
                name: name.trim().to_string(),
                value: Some(value.trim().to_string()),
                source: Some(path.as_ref().to_path_buf()),
                line_number: Some(index + 1),
            });
        }
    }

    EnvFile {
        path: path.as_ref().to_path_buf(),
        keys,
    }
}
