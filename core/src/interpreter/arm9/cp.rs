use crate::{Core, Interpreter};

pub fn qarith<const ARG: arm_decode::QArith>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn dsp_mul<const ARG: arm_decode::DspMul>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn cdp(_core: &mut Core<Interpreter>, _instr: u32) {}

pub fn cp_mov<const ARG: arm_decode::CPMov>(_core: &mut Core<Interpreter>, _instr: u32) {
    unimplemented!()
}

pub fn cp_transfer<const ARG: arm_decode::CPTransfer>(_core: &mut Core<Interpreter>, _instr: u32) {
    unimplemented!()
}
