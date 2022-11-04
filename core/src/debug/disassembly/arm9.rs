mod branch;
mod coproc;
mod data;
mod mem;
mod misc;

use crate::{
    core::{arm9::bus, bus::DebugAccess, engine::Engine},
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
    pub fn cond_print(cond: u32) -> &'static str {
        debug_assert!(cond < 0x10);
        match cond {
            0x0 => "eq",
            0x1 => "ne",
            0x2 => "cs",
            0x3 => "cc",
            0x4 => "mi",
            0x5 => "pl",
            0x6 => "vs",
            0x7 => "vc",
            0x8 => "hi",
            0x9 => "ls",
            0xA => "ge",
            0xB => "lt",
            0xC => "gt",
            0xD => "le",
            0xE => "",
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn cond(instr: u32) -> &'static str {
        cond_print((instr >> 28) & 0xF)
    }
}

type DecodeFn = fn(u32, u32) -> String;
static COND_LUT: [DecodeFn; 1 << 12] = include!("../../../gen/arm9_arm_lut");

pub fn disassemble_arm9<E: Engine>(core: &mut Core<E>, adr: u32) -> (String, Vec<u8>) {
    let instr = bus::read32::<DebugAccess, _>(core, adr);
    let byte_vec = vec![
        instr as u8,
        (instr >> 8) as u8,
        (instr >> 16) as u8,
        (instr >> 24) as u8,
    ];
    let index = (((instr >> 16) & 0xFF0) | ((instr >> 4) & 0xF)) as usize;
    let str = if instr >> 28 & 0xF == 0xF {
        if instr >> 25 & 0b101 == 0b101 {
            branch::blx::<true>(adr, instr)
        } else {
            misc::undef(adr, instr)
        }
    } else {
        (COND_LUT[index])(adr, instr)
    };
    (str, byte_vec)
}
