use colored::Colorize;
use std::fmt::Display;

#[derive(Clone)]
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger {
    pub fn fatal(self, msg: impl Display) -> ! {
        panic!("[{}] {}", "ERROR".red(), msg)
    }

    pub fn warning(&self, msg: impl Display) {
        println!("[{}] {}", "WARNING".yellow(), msg);
    }

    pub fn info(&self, msg: impl Display) {
        println!("[{}] {}", "INFO".green(), msg);
    }
}
