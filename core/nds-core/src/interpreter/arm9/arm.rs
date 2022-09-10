mod branch;
mod dp;
mod load_store;
mod misc;

use crate::{core::Core, interpreter::Interpreter};
use bitmatch_proc::bitmatch;

pub type ARMFn = fn(&mut Core<Interpreter>, u32);

const LUT_SIZE: usize = 0;

fn unimplemented(_: &mut Core<Interpreter>, instr: u32) {
    panic!("unimplemented! instr: 0x{instr:08X?}");
}

static INSTR_LUT: [ARMFn; LUT_SIZE] = [];

#[inline]
pub fn execute(core: &mut Core<Interpreter>, instr: u32) {
    let f = INSTR_LUT[((((instr >> 12) & 0xFF0) | ((instr >> 4) & 0xF)) & 0xFFF) as usize];
    (f)(core, instr);
}
