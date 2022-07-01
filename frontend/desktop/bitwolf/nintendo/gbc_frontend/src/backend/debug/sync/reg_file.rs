use super::Emu;
use crate::state::substates;
use gbc_backend::engines::interpreter::{self, Interpreter};

pub(super) fn get(emu: &mut Emu<Interpreter>) -> substates::RegisterFile {
    use interpreter::debug::registers::{Flag, R16};
    let pc = interpreter::debug::registers::get_pc(emu);
    let sp = interpreter::debug::registers::get_sp(emu);
    let af = interpreter::debug::registers::get(emu, R16::AF);
    let bc = interpreter::debug::registers::get(emu, R16::BC);
    let de = interpreter::debug::registers::get(emu, R16::DE);
    let hl = interpreter::debug::registers::get(emu, R16::HL);
    let z = interpreter::debug::registers::get_flag(emu, Flag::Z);
    let n = interpreter::debug::registers::get_flag(emu, Flag::N);
    let h = interpreter::debug::registers::get_flag(emu, Flag::H);
    let c = interpreter::debug::registers::get_flag(emu, Flag::C);
    substates::RegisterFile {
        pc,
        sp,
        af,
        bc,
        de,
        hl,
        z,
        n,
        h,
        c,
    }
}
