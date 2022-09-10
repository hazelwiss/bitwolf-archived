use crate::{
    cpu::arm9::ARM9,
    engine::Engine,
    rom::{self, parse_rom},
};
use core_util::{kb, mem::byte_cell::BytesCell};
use rom::Cartridge;
use util::Logger;

#[derive(Debug)]
pub enum BuildError {}

pub struct Builder {
    pub rom: alloc::vec::Vec<u8>,
    #[cfg(feature = "log")]
    pub logger: Logger,
}

impl Builder {
    pub fn build<E: Engine>(self) -> Result<Core<E>, BuildError> {
        let (arm9_data, arm7_data, global_data) = E::into_data();
        let arm9 = ARM9 {
            arm9_data,
            #[cfg(feature = "log")]
            logger: Logger::default(),
            //reg_file: Default::default(),
        };
        let mut core = Core {
            arm9,
            #[cfg(feature = "log")]
            log: Logger::default(),
            // deliberatelly break safety due to the `reset` function otherwise doubly allocating.
            main_memory: unsafe { BytesCell::from_raw(core::ptr::null_mut()) },
        };
        core.init();
        Ok(core)
    }
}

pub struct Core<E: Engine> {
    pub arm9: ARM9<E>,
    #[cfg(feature = "log")]
    pub log: Logger,
    main_memory: BytesCell<{ kb!(8192) }>,
}

impl<E: Engine> Core<E> {
    pub fn init(&mut self) {
        // reset main memory
        self.main_memory = BytesCell::new_zeroed();
        self.arm9.reset();
        // set arm9 pc to correct exection address.
        //self.arm9
        //    .reg_file
        //    .set_pc(self.cartridge.header.arm9_entry_address);
    }
}
