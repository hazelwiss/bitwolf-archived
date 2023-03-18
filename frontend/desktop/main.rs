#![allow(dead_code)]
#![allow(unused)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate log;

mod cla;
mod config;
mod core;
mod gui;
mod state;


fn main() {
    // Initializes logging for all cores and frontend.
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    gui::main()
}
