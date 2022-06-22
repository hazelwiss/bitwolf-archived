use crate::{io::IOReg, memory_map::Section, Bus};

impl Bus {
    pub fn read(&mut self, adr: u16) -> u8 {
        let section = Section::from_adr(adr);
        match section {
            Section::ROM0(offset) => self.read_rom0(offset as usize),
            Section::ROM1(offset) => self.read_rom1(offset as usize),
            Section::VRAM(offset) => self.read_vram(offset as usize),
            Section::ERAM(offset) => self.read_eram(offset as usize),
            Section::WRAM0(offset) => self.read_wram0(offset as usize),
            Section::WRAM1(offset) => self.read_wram1(offset as usize),
            Section::MIRROR(offset) => self.read_mirror(offset as usize),
            Section::OAM(offset) => self.read_oam(offset as usize),
            Section::Unusable(offset) => self.read_unusable(offset as usize),
            Section::IO(index) => self.read_io(index),
            Section::HRAM(offset) => self.read_hram(offset as usize),
            Section::Invalid(offset) => logger::fatal!("read to invalid memory {offset:04X}"),
        }
    }

    #[inline]
    fn read_rom0(&mut self, offset: usize) -> u8 {
        if offset == 0x100 {
            for i in 0..256 {
                self.rom0[i] = self.rom_256bytes[i];
            }
        }
        self.rom0[offset]
    }

    #[inline]
    fn read_rom1(&self, offset: usize) -> u8 {
        self.rom1[offset]
    }

    #[inline]
    fn read_vram(&self, offset: usize) -> u8 {
        self.ppu.read_vram(offset)
    }

    #[inline]
    fn read_eram(&self, offset: usize) -> u8 {
        self.eram[offset]
    }

    #[inline]
    fn read_wram0(&self, offset: usize) -> u8 {
        self.wram0[offset]
    }

    #[inline]
    fn read_wram1(&self, offset: usize) -> u8 {
        self.wram1[offset]
    }

    #[inline]
    fn read_mirror(&self, _offset: usize) -> u8 {
        todo!()
    }

    #[inline]
    fn read_oam(&self, offset: usize) -> u8 {
        self.ppu.read_oam(offset)
    }

    #[inline]
    fn read_unusable(&self, offset: usize) -> u8 {
        logger::fatal!("Attempting to read from unusable section 0x{offset:04X}")
    }

    #[inline]
    fn read_io(&mut self, index: u8) -> u8 {
        self.io.read(&mut self.ppu, IOReg::from_index(index))
    }

    #[inline]
    fn read_hram(&self, offset: usize) -> u8 {
        self.hram[offset]
    }
}
