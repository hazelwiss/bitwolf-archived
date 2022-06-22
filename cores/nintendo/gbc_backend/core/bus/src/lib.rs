pub mod access;
pub mod io;
pub mod memory_map;

pub struct Bus {
    ppu: ppu::PPU,
    rom_256bytes: [u8; 256],
    rom0: [u8; 0x4000],
    rom1: [u8; 0x4000],
    eram: [u8; 0x2000],
    wram0: [u8; 0x1000],
    wram1: [u8; 0x1000],
    io: io::IO,
    hram: [u8; 0x7E],
}

impl Bus {
    pub fn new(bootrom: [u8; 256], rom: Vec<u8>) -> Self {
        if rom.len() > 0x8000 {
            logger::fatal!("ROM too large!");
        }
        if rom.len() < 256 {
            logger::fatal!("ROM too smal!");
        }
        let mut rom0 = [0; 0x4000];
        let mut rom1 = [0; 0x4000];
        let mut rom_256bytes = [0; 256];
        for i in 0..256 {
            rom0[i] = bootrom[i];
            rom_256bytes[i] = rom[i];
        }
        for i in 0x100..0x4000 {
            if i >= rom.len() {
                break;
            }
            rom0[i] = rom[i];
        }
        for i in 0x4000..0x8000 {
            if i >= rom.len() {
                break;
            }
            rom1[i - 0x4000] = rom[i];
        }
        Self {
            ppu: ppu::PPU::new(),
            rom0,
            rom1,
            rom_256bytes,
            eram: [0; 0x2000],
            wram0: [0; 0x1000],
            wram1: [0; 0x1000],
            io: io::IO::new(),
            hram: [0; 0x7E],
        }
    }
}
