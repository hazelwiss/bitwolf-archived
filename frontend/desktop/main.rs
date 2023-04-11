#![allow(dead_code)]
#![allow(unused)]
#![feature(atomic_bool_fetch_not)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate log;

mod cla;
mod config;
mod frontend;
mod gui;

fn main() {
    // Initializes logging for all cores and frontend.
    env_logger::builder()
        .filter_level(log::LevelFilter::Off)
        .init();
    gui::main()
}
