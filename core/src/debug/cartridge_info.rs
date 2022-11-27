pub use crate::cartridge::Header;
use crate::{Core, Engine};

pub fn cartridge_header<E: Engine>(core: &Core<E>) -> Header {
    core.cartidge_header.clone()
}
