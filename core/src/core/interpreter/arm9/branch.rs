use crate::{Core, Interpreter};

pub fn blx<const IMM: bool>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn bx(core: &mut Core<Interpreter>, instr: u32) {}

pub fn b<const ARG: arm_decode::B>(core: &mut Core<Interpreter>, instr: u32) {}
