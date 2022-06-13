pub(crate) mod access;
pub(crate) mod memory_map;

use crate::core::ppu;

pub(crate) struct Bus {
    _ppu: ppu::PPU,
    _bootrom: [u8; 256],
}

impl Bus {
    pub fn new(bootrom: [u8; 256], _rom: Vec<u8>) -> Self {
        Self {
            _ppu: ppu::PPU::new(),
            _bootrom: bootrom,
        }
    }
}
