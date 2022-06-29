pub mod debug;

mod binder;
mod instructions;

use crate::{Emu, Engine};

pub struct Interpreter;

impl Engine for Interpreter {
    type EngineData = ();
}

pub fn step(emu: &mut Emu<Interpreter>) {
    emu.interrupt_handler();
    emu.fetch_decode_execute();
}
