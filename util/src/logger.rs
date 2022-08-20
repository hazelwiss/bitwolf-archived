use colored::{ColoredString, Colorize};
use std::{fmt::Display, path::PathBuf};

#[derive(Clone)]
pub struct Logger {
    pub display_str: Option<&'static str>,
    pub output_file: Option<PathBuf>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            display_str: None,
            output_file: None,
        }
    }
}

pub fn construct_message(logger: &Logger, kind: ColoredString, msg: impl Display) -> String {
    format!(
        "[{}] {}",
        kind,
        match logger.display_str {
            Some(display_str) => format!("{}: {}", display_str, msg),
            None => msg.to_string(),
        }
    )
}

impl Logger {
    pub fn fatal(self, msg: impl Display) -> ! {
        let msg = construct_message(&self, "ERROR".red(), msg);
        panic!("{msg}");
    }

    pub fn warning(&self, msg: impl Display) {
        let msg = construct_message(self, "WARNING".yellow(), msg);
        println!("{msg}");
    }

    pub fn info(&self, msg: impl Display) {
        let msg = construct_message(self, "INFO".green(), msg);
        println!("{msg}");
    }
}
