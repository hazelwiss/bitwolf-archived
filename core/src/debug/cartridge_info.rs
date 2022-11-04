pub use crate::core::cartridge::Header;
use crate::core::{engine::Engine, Core};

pub fn cartridge_header<E: Engine>(core: &Core<E>) -> Header {
    core.cartidge_header.clone()
}
