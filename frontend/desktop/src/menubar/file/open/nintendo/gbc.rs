use crate::backends::nintendo::gbc::{Context, Engine};
use gbc::{
    engines::interpreter::{Builder, Interpreter},
    Core,
};
use std::path::Path;

pub fn open(_: &Path) -> Context {
    let core = Core::<Interpreter>::new(Builder {});
    Context::new(Engine::Interp(core))
}
