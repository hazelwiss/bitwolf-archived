use crate::backends::nintendo::gbc::Context;
use gbc::{
    engines::interpreter::{Builder, Interpeter},
    Core,
};
use std::path::Path;

pub fn open(_: &Path) -> Context {
    let mut core = Core::<Interpeter>::new(Builder {});
    core.step();
    Context {}
}
