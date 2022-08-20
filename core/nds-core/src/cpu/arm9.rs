mod decode;

use crate::engine::Engine;
#[cfg(feature = "log")]
use util::Logger;

pub struct ARM9<E: Engine> {
    pub(crate) engine_data: E::EngineData,
    #[cfg(feature = "log")]
    pub(crate) logger: Logger,
}
