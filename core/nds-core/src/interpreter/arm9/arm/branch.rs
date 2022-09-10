use crate::{core::Core, interpreter::Interpreter};

pub fn b(core: &mut Core<Interpreter>, instr: u32) {
    core.log.info("b");
}

pub fn bl(core: &mut Core<Interpreter>) {}

pub fn bx(core: &mut Core<Interpreter>, instr: u32) {
    core.log.info("bx");
}

pub fn blx_imm(core: &mut Core<Interpreter>, instr: u32) {
    core.log.info("blx");
}

pub fn blx_reg(core: &mut Core<Interpreter>, instr: u32) {}
