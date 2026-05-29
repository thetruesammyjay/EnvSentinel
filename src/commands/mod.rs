pub mod diff;
pub mod init;
pub mod scan;
pub mod validate;
pub mod watch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    pub exit_code: i32,
    pub message: String,
}

impl CommandOutcome {
    pub fn new(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
        }
    }
}
