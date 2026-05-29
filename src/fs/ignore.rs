use std::path::{Path, PathBuf};

pub fn should_ignore(path: impl AsRef<Path>, ignore_directories: &[PathBuf]) -> bool {
    let path = path.as_ref();

    path.components().any(|component| {
        let component = component.as_os_str();
        ignore_directories.iter().any(|ignored| {
            ignored
                .file_name()
                .map(|ignored_name| ignored_name == component)
                .unwrap_or(false)
        })
    })
}
