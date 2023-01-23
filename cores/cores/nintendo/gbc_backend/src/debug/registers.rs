use crate::{Core, Engine};

pub use crate::core::cpu::registers::{Flag, R16};

pub fn get<E: Engine>(core: &Core<E>, reg: R16) -> u16 {
    core.cpu.r16_get(reg)
}

pub fn get_pct<E: Engine>(core: &Core<E>) -> u16 {
    core.cpu.pc_get()
}

pub fn get_spt<E: Engine>(core: &Core<E>) -> u16 {
    core.cpu.sp_get()
}

pub fn get_flagt<E: Engine>(core: &Core<E>, flag: Flag) -> bool {
    core.cpu.flag_get(flag)
}
