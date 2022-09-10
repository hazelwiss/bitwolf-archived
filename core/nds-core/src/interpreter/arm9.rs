use super::Interpreter;
use crate::core::Core;

pub mod arm;
pub mod thumb;

pub(crate) fn step_arm9(core: &mut Core<Interpreter>) {
    let instr = 0;
    arm::execute(core, instr)
}
