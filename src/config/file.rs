use std::path::Path;

use super::defaults::Defaults;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFile {
    pub defaults: Defaults,
}

impl ConfigFile {
    pub fn load(_path: impl AsRef<Path>) -> Self {
        Self {
            defaults: Defaults::default(),
        }
    }
}
