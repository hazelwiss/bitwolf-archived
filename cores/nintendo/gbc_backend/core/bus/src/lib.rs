pub mod access;
pub mod memory_map;

pub struct Bus {
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
