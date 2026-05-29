pub mod app;
pub mod cli;
pub mod commands;
pub mod config;
pub mod env;
pub mod fs;
pub mod report;
pub mod util;

pub fn run() -> i32 {
    app::run()
}
