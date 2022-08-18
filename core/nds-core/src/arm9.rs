mod instructions;
mod registers;

use crate::engine::Engine;
#[cfg(feature = "logging")]
use util::Logger;

pub struct ARM9<E: Engine> {
    engine_data: E::EngineData,
    #[cfg(feature = "logging")]
    logger: Logger,
}

impl<E: Engine> ARM9<E> {
    pub fn new() -> Self {
        Self {
            engine_data: E::EngineData::default(),
            #[cfg(feature = "logging")]
            logger: Logger::new(),
        }
    }
}
