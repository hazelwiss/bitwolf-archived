pub mod access;
pub mod io;
pub mod memory_map;

pub struct Bus {
    ppu: ppu::PPU,
    _bootrom: [u8; 256],
    rom0: [u8; 0x4000],
    rom1: [u8; 0x4000],
    eram: [u8; 0x2000],
    wram0: [u8; 0x1000],
    wram1: [u8; 0x1000],
    io: io::IO,
    hram: [u8; 0x7E],
}

impl Bus {
    pub fn new(bootrom: [u8; 256], _rom: Vec<u8>) -> Self {
        Self {
            ppu: ppu::PPU::new(),
            _bootrom: bootrom,
            rom0: [0; 0x4000],
            rom1: [0; 0x4000],
            eram: [0; 0x2000],
            wram0: [0; 0x1000],
            wram1: [0; 0x1000],
            io: io::IO::new(),
            hram: [0; 0x7E],
        }
    }
}
