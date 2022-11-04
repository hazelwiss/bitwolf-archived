use crate::{Core, Interpreter};

pub fn transfer<const ARG: arm_decode::Transfer>(core: &mut Core<Interpreter>, instr: u32) {}

pub fn misc_transfer<const ARG: arm_decode::MiscTransfer>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
}

pub fn transfer_multiple<const ARG: arm_decode::TransferMult>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
}

pub fn transfer_double<const ARG: arm_decode::TransferDouble>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
}
