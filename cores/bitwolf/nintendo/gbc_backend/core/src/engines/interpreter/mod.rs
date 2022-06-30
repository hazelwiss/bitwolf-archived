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

pub fn frame<F: FnMut(crate::Texture, &mut Emu<Interpreter>)>(
    emu: &mut Emu<Interpreter>,
    mut callback: F,
) {
    if let Some(frame) = emu.bus.ppu.present_frame() {
        callback(frame.clone(), emu);
        emu.bus.ppu.invalidate_frame();
    }
}

pub fn run_until_frame<F: FnMut(crate::Texture, &mut Emu<Interpreter>)>(
    emu: &mut Emu<Interpreter>,
    mut callback: F,
) {
    loop {
        if let Some(frame) = emu.bus.ppu.present_frame() {
            callback(frame.clone(), emu);
            emu.bus.ppu.invalidate_frame();
            break;
        } else {
            step(emu);
        }
    }
}
