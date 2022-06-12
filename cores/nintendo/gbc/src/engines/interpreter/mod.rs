use super::Engine;
use crate::Core;

pub struct Builder {}

pub struct Interpeter {}

impl Engine for Interpeter {}

impl Core<Interpeter> {
    pub fn new(_: Builder) -> Self {
        Self {
            engine: Interpeter {},
        }
    }

    pub fn step(&mut self) {
        println!("step!");
    }
}
