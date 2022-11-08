mod branch;
mod cp;
mod data;
mod mem;
mod misc;

use crate::{
    cpu::{arm9::bus, bus::CPUAccess},
    Core, Interpreter,
};
use arm_decode::*;

type CondFn = fn(&mut Core<Interpreter>, u32);
static COND_LUT: [CondFn; 1 << 12] = include!("../../gen/arm9_arm_lut");

#[inline]
pub fn step(core: &mut Core<Interpreter>) {
    let instr = bus::read32::<CPUAccess, _>(core, core.arm9.registers.get_pc());
    core.arm9
        .registers
        .set_pc(core.arm9.registers.get_pc().wrapping_add(4));
    if (instr >> 28) & 0xF == 0xF {
        if (instr >> 25) & 0b111 == 0b101 {
            branch::blx::<true>(core, instr)
        } else {
            misc::undef(core, instr);
        }
    } else {
        let index = ((instr >> 4) & 0xF) | ((instr >> 16) & 0xFF0);
        COND_LUT[index as usize](core, instr)
    }
}

#[inline]
pub fn run(core: &mut Core<Interpreter>, cycles: u64) {
    for _ in 0..cycles {
        step(core)
    }
}
