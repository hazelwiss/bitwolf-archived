mod decode;

use crate::engine::Engine;
#[cfg(feature = "debug-log")]
use core_util::logger::Logger;

pub struct ARM9<E: Engine> {
    engine_data: E::EngineData,
    #[cfg(feature = "debug-log")]
    logger: Logger,
}
