use super::Engine;
use crate::Core;

pub struct Builder {}

pub struct Interpreter {}

impl Engine for Interpreter {}

impl Core<Interpreter> {
    pub fn new(_: Builder) -> Self {
        Self {
            engine: Interpreter {},
        }
    }

    pub fn step(&mut self) {
        println!("step!");
    }
}
