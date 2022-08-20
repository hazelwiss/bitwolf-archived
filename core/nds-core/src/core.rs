use crate::{cpu::arm9::ARM9, engine::Engine};
use core_util::{kb, mem::byte_cell::OwnedBytesCell};
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
        let engine_data = E::EngineData::default();
        let arm9 = ARM9 {
            engine_data,
            #[cfg(feature = "log")]
            logger: Logger::default(),
        };
        Ok(Core {
            arm9,
            #[cfg(feature = "log")]
            logger: Logger::default(),
            main_memory: OwnedBytesCell::new_zeroed(),
        })
    }
}

pub struct Core<E: Engine> {
    pub arm9: ARM9<E>,
    #[cfg(feature = "log")]
    logger: Logger,
    main_memory: OwnedBytesCell<{ kb!(8192) }>,
}
