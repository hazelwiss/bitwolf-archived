use crate::{Core, Interpreter};

pub fn dp<const ARG: arm_decode::Dp>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn clz(core: &mut Core<Interpreter>, instr: u32) {}

pub fn msr<const ARG: arm_decode::Msr>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn mrs<const ARG: arm_decode::Mrs>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn mul<const ARG: arm_decode::Mul>(core: &mut Core<Interpreter>, instr: u32) {}
