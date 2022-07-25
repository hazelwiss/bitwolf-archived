use super::{Emu, Interpreter};

pub use crate::core::cpu::registers::{Flag, R16};

pub fn get(emu: &Emu<Interpreter>, reg: R16) -> u16 {
    emu.r16_get(reg)
}

pub fn get_pc(emu: &Emu<Interpreter>) -> u16 {
    emu.pc_get()
}

pub fn get_sp(emu: &Emu<Interpreter>) -> u16 {
    emu.sp_get()
}

pub fn get_flag(emu: &Emu<Interpreter>, flag: Flag) -> bool {
    emu.flag_get(flag)
}
