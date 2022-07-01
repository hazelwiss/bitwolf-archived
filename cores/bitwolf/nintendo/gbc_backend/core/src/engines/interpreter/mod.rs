pub mod debug;

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
    max_steps: Option<u64>,
    mut callback: F,
) {
    let mut steps = 0;
    loop {
        if let Some(frame) = emu.bus.ppu.present_frame() {
            callback(frame.clone(), emu);
            emu.bus.ppu.invalidate_frame();
            break;
        } else if let Some(max_steps) = max_steps {
            if steps < max_steps {
                step(emu);
            } else {
                break;
            }
            steps += 1;
        } else {
            step(emu)
        }
    }
}
