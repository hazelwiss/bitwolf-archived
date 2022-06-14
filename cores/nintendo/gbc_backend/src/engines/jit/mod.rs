use super::Engine;
use crate::Core;

pub struct Builder {}

pub struct JIT {}

impl Engine for JIT {}

impl Core<JIT> {
    pub fn new(_: Builder) -> Self {
        Self { engine: JIT {} }
    }
}
