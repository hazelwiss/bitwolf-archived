mod branch;
mod data;
mod mem;
mod misc;

use crate::{
    core::{arm9::bus, bus::DebugAccess},
    Core,
};
use alloc::{string::String, vec::Vec};
use arm_decode::*;

fn reg_print(reg: u32) -> String {
    assert!(reg < 0x10);
    format!("r{reg}")
}

type DecodeFn = fn(u32) -> String;
static COND_LUT: [DecodeFn; 1 << 12] = include!("../../../gen/arm9_arm_lut");

pub fn disassemble_arm9(core: &mut Core, adr: u32) -> (String, Vec<u8>) {
    let instr = bus::read32::<DebugAccess>(core, adr);
    let byte_vec = vec![
        (instr >> 24) as u8,
        (instr >> 16) as u8,
        (instr >> 8) as u8,
        instr as u8,
    ];
    let index = (((instr >> 16) & 0xFF0) | ((instr >> 4) & 0xF)) as usize;
    let str = if instr >> 28 & 0xF == 0xF {
        if instr >> 25 & 0b101 == 0b101 {
            branch::blx::<true>(instr)
        } else {
            misc::undef(instr)
        }
    } else {
        (COND_LUT[index])(instr)
    };
    (str, byte_vec)
}
