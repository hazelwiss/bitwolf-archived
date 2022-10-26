mod branch;
mod coproc;
mod data;
mod mem;
mod misc;

use crate::{
    core::{arm9::bus, bus::DebugAccess},
    Core,
};
use alloc::{string::String, vec::Vec};
use arm_decode::*;

mod common {
    pub use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[inline]
    pub fn reg(reg: u32) -> String {
        debug_assert!(reg < 0x10);
        match reg & 0xF {
            v @ 0..=12 => format!("r{v}"),
            13 => "sp".to_string(),
            14 => "lr".to_string(),
            15 => "pc".to_string(),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn cond_print(cond: u32) -> String {
        debug_assert!(cond < 0x10);
        format!("")
    }

    #[inline]
    pub fn cond(instr: u32) -> String {
        cond_print((instr >> 28) & 0xF)
    }
}

type DecodeFn = fn(u32) -> String;
static COND_LUT: [DecodeFn; 1 << 12] = include!("../../../gen/arm9_arm_lut");

pub fn disassemble_arm9(core: &mut Core, adr: u32) -> (String, Vec<u8>) {
    let instr = bus::read32::<DebugAccess>(core, adr);
    let byte_vec = vec![
        instr as u8,
        (instr >> 8) as u8,
        (instr >> 16) as u8,
        (instr >> 24) as u8,
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
