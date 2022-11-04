use crate::{Core, Interpreter};

pub fn qarith<const ARG: arm_decode::QArith>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn dsp_mul<const ARG: arm_decode::DspMul>(core: &mut Core<Interpreter>, instr: u32) {}
