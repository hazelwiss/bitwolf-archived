pub use jit::{Builder, JIT};

use super::Engine;
use crate::Core;

impl Engine for JIT {}

impl Core<JIT> {
    pub fn new(_: Builder) -> Self {
        Self { engine: JIT {} }
    }
}
