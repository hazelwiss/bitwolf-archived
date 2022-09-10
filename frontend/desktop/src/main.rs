#![allow(unused)]

mod common;
mod cores;
mod ui;

use cores::{Core, CoreType};
use std::{fmt::Display, path::PathBuf};
use util::Logger;

pub struct Ctx {
    previously_loaded_files: Vec<(PathBuf, CoreType)>,
    config_window_active: bool,
    help_window_active: bool,
    #[cfg(feature = "log")]
    logger: Logger,
    fullscreen: bool,
}

fn main() {
    ui::main();
}
