pub mod decode;

mod registers;

use crate::engine::Engine;
#[cfg(feature = "log")]
use util::Logger;

pub struct ARM9<E: Engine> {
    pub arm9_data: E::ARM9Data,
    #[cfg(feature = "log")]
    pub logger: Logger,
    //pub reg_file: registers::RegFile,
}

impl<E: Engine> ARM9<E> {
    pub fn reset(&mut self) {}
}
