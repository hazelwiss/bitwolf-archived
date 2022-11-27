pub mod bus;

use crate::{
    cpu::registers::{Psr, RegFile},
    Core, Engine,
};
use alloc::boxed::Box;
use bus::ptrs::masks;

#[allow(clippy::new_without_default)]
pub struct Arm9<E: Engine> {
    pub registers: RegFile,
    pub(crate) cpsr: Psr,
    pub(crate) bus_ptrs: Box<bus::ptrs::Ptrs>,
    engine_data: E::Arm9Data,
}

impl<E: Engine> Default for Arm9<E> {
    fn default() -> Self {
        Self {
            registers: RegFile { gpr: [0; 16] },
            cpsr: Psr::new(),
            bus_ptrs: Box::default(),
            engine_data: Default::default(),
        }
    }
}

impl<E: Engine> Arm9<E> {
    pub fn reset(core: &mut Core<E>) {
        core.arm9
            .registers
            .set_pc(core.cartidge_header.arm9_entry());
        core.arm9.bus_ptrs.map(
            0x02000000,
            mb!(4),
            masks::R | masks::W_16_32 | masks::W_8,
            core.main_memory.as_mut_ptr(),
        )
    }

    #[inline(always)]
    pub fn set_reg(&mut self, reg_index: u32, val: u32) {
        debug_assert!(reg_index < 0x10);
        self.registers.set(reg_index as usize & 0xF, val);
    }

    #[inline(always)]
    pub fn get_reg(&self, reg_index: u32) -> u32 {
        debug_assert!(reg_index < 0x10);
        self.registers.get(reg_index as usize & 0xF)
    }
}
