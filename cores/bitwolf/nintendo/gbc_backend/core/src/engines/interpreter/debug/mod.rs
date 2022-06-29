use super::{Emu, Interpreter};
use crate::engines::debug;

pub fn step(emu: &mut Emu<Interpreter>) {
    super::step(emu);
}

pub fn step_over(emu: &mut Emu<Interpreter>) {
    super::step(emu);
}

pub fn get_pc_value(emu: &mut Emu<Interpreter>) -> u16 {
    emu.pc_get()
}

pub fn debug_read(emu: &Emu<Interpreter>, adr: u16) -> u8 {
    crate::bus::debug::read::read(&emu.bus, adr)
}

pub fn debug_write(_emu: &mut Emu<Interpreter>, _adr: u16, _val: u8) {
    todo!()
}

pub fn disassemle(
    emu: &Emu<Interpreter>,
    adr: u16,
) -> common_core::disassembly::DisassembledOutput {
    debug::disassembly::disassmble_at_adr(emu, adr)
}
