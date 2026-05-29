use std::path::PathBuf;

pub fn discover_candidates(_root: impl Into<PathBuf>) -> Vec<PathBuf> {
    vec![PathBuf::from(".env"), PathBuf::from(".env.example"), PathBuf::from(".env.local")]
}
