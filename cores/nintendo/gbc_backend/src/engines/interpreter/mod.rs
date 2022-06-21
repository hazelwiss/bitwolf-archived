pub use interpreter::{Builder, Interpreter};

use super::Engine;
use crate::Core;

impl Engine for Interpreter {}

impl Core<Interpreter> {
    pub fn new(builder: Builder) -> Self {
        Self {
            engine: Interpreter::new(builder),
        }
    }

    pub fn step(&mut self) {
        self.engine.fetch_decode_execute();
    }
}
