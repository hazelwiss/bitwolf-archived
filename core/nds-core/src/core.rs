use crate::{
    cpu::arm9::ARM9,
    engine::Engine,
    rom::{self, parse_rom},
};
use core_util::{kb, mem::byte_cell::OwnedBytesCell};
use rom::Cartridge;
use util::Logger;

#[derive(Debug)]
pub enum BuildError {}

pub struct Builder {
    pub rom: Vec<u8>,
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
        };
        let mut core = Core {
            arm9,
            #[cfg(feature = "log")]
            log: Logger::default(),
            main_memory: OwnedBytesCell::new_zeroed(),
            cartridge: Cartridge::default(),
        };
        core.cartridge = Cartridge::from_rom(self.rom);
        Ok(core)
    }
}

pub struct Core<E: Engine> {
    pub arm9: ARM9<E>,
    #[cfg(feature = "log")]
    log: Logger,
    main_memory: OwnedBytesCell<{ kb!(8192) }>,
    pub(crate) cartridge: Cartridge,
}
