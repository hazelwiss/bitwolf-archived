use super::{cond, reg};
use alloc::string::String;

pub fn blx<const IMM: bool>(instr: u32) -> String {
    format!(
        "blx {}",
        if IMM {
            let offset = (((instr as i32 & 0xFFFFFF) << 8) >> 8) * 4;
            format!("#0x{offset:X}")
        } else {
            reg(instr & 0xF)
        }
    )
}

pub fn bx(instr: u32) -> String {
    let rm = reg(instr & 0xF);
    let cond = cond(instr);
    format!("bx{cond} {rm}")
}

pub fn b<const ARG: arm_decode::B>(instr: u32) -> String {
    let offset = (((instr as i32 & 0xFFFFFF) << 8) >> 8) * 4;
    let cond = cond(instr);
    format!("b{cond}{} #0x{offset:X}", if ARG.link { "l" } else { "" })
}
