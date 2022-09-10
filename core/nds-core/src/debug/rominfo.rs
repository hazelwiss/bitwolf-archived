pub use crate::rom::{Cartridge, CartridgeHeader};
use crate::{core::Core, engine::Engine};

pub fn read_cartridge<E: Engine>(core: &Core<E>) -> &Cartridge {
    todo!() //&core.cartridge
}

pub fn cartridge_header<E: Engine>(core: &Core<E>) -> &CartridgeHeader {
    &read_cartridge(core).header
}
