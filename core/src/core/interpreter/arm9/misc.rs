use crate::{Core, Interpreter};

pub fn undef(core: &mut Core<Interpreter>, _: u32) {}

pub fn unpred(core: &mut Core<Interpreter>, _: u32) {}

pub fn bkpt(core: &mut Core<Interpreter>, _: u32) {}

pub fn swi(core: &mut Core<Interpreter>, _: u32) {}
