mod decode;

use crate::engine::Engine;
#[cfg(feature = "log")]
use util::Logger;

pub struct ARM9<E: Engine> {
    pub(crate) arm9_data: E::ARM9Data,
    #[cfg(feature = "log")]
    pub(crate) logger: Logger,
}
