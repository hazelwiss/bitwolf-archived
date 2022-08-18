pub mod engine;

mod arm9;
#[cfg(test)]
mod test;

use arm9::ARM9;
use engine::Engine;
use util::Logger;

pub struct Builder {
    pub rom: Vec<u8>,
    #[cfg(feature = "logging")]
    pub logger: Logger,
}

pub struct Core<E: Engine> {
    arm9: ARM9<E>,
    #[cfg(feature = "logging")]
    logger: Logger,
}

impl<E: Engine> Core<E> {
    pub fn new(builder: Builder) -> Self {
        Self {
            arm9: ARM9::new(),
            #[cfg(feature = "logging")]
            logger: builder.logger,
        }
    }
}
