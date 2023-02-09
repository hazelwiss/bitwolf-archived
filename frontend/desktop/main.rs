#![allow(dead_code)]
#![allow(unused)]
#![feature(let_chains)]

extern crate libfrontend as lib;

#[macro_use]
extern crate log;

mod cla;
mod config;
mod core;
mod debug;
mod gui;

fn main() {
    // Initializes logging for all cores and frontend.
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    gui::main()
}
