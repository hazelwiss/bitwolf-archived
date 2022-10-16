pub use crate::core::cartridge::Header;
use crate::core::Core;

pub fn cartridge_header(core: &Core) -> Header {
    core.cartidge_header.clone()
}
