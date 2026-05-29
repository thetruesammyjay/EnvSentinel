use std::path::Path;

pub fn should_ignore(path: impl AsRef<Path>) -> bool {
    matches!(
        path.as_ref().to_string_lossy().as_ref(),
        "node_modules" | "target" | "dist"
    )
}
