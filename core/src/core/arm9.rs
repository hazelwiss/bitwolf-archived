pub mod bus;

use self::bus::ptrs::masks;
use super::{engine::Engine, registers::RegFile};
use crate::Core;
use alloc::boxed::Box;

#[allow(clippy::new_without_default)]
pub struct Arm9<E: Engine> {
    pub registers: RegFile,
    pub bus_ptrs: Box<bus::ptrs::Ptrs>,
    engine_data: E::Arm9Data,
}

impl<E: Engine> Default for Arm9<E> {
    fn default() -> Self {
        Self {
            registers: RegFile { gpr: [0; 16] },
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
}
