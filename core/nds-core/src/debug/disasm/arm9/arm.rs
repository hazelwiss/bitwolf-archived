mod branch;
mod dp;
mod load_store;
mod misc;

use crate::cpu::arm9::decode::arm::{decode_cond, Instr};

pub type ARMFn = fn(u32);

const LUT_SIZE: usize = 0x1000;

fn unimplemented(instr: u32) {
    todo!()
}

static INSTR_LUT: [ARMFn; LUT_SIZE] = const {
    let mut instr_lut: [ARMFn; LUT_SIZE] = [unimplemented; LUT_SIZE];
    let mut i = 0;
    //while i < LUT_SIZE {
    //    let instr = ((i & 0xFF0) << 16) as u32 | ((i & 0xF) << 4) as u32;
    //    let instr_type = decode_cond(instr);
    //    instr_lut[i] = match instr_type {
    //        Instr::Unimplemented => todo!(),
    //    };
    //    i += 1;
    //}
    instr_lut
};

#[inline]
pub fn disasm(instr: u32) {
    let f = INSTR_LUT[((((instr >> 12) & 0xFF0) | ((instr >> 4) & 0xF)) & 0xFFF) as usize];
    (f)(instr);
}
