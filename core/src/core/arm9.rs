pub mod bus;

use self::bus::ptrs::masks;
use super::registers::RegFile;
use crate::Core;
use alloc::boxed::Box;

#[allow(clippy::new_without_default)]
pub struct ARM9 {
    pub registers: RegFile,
    pub bus_ptrs: Box<bus::ptrs::Ptrs>,
}

impl Default for ARM9 {
    fn default() -> Self {
        Self {
            registers: RegFile { gpr: [0; 16] },
            bus_ptrs: Box::default(),
        }
    }
}

impl ARM9 {
    pub fn reset(core: &mut Core) {
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
