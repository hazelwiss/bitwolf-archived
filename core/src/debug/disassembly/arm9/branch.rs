use alloc::string::String;
use arm_decode::*;

pub fn blx<const IMM: bool>(instr: u32) -> String {
    format!("blx")
}

pub fn bx(instr: u32) -> String {
    format!("bx")
}

pub fn b<const LINK: bool>(instr: u32) -> String {
    format!("b")
}
