use crate::{core::Core, engine::Engine};

pub mod arm;
pub mod thumb;

pub(crate) fn step_arm9<E: Engine>(core: &mut Core<E>) {}
