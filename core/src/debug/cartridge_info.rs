pub use crate::cartridge::Header;
use crate::{engine::Engine, Core};

pub fn cartridge_header<E: Engine>(core: &Core<E>) -> Header {
    core.cartidge_header.clone()
}
