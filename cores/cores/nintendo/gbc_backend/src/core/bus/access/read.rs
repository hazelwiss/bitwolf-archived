use crate::core::{
    bus::{address_space, io::IOReg, memory_map::Section, Bus},
    ppu::PPU,
};

impl Bus {
    pub fn read(&mut self, adr: u16) -> u8 {
        let section = Section::from_adr(adr);
        match section {
            Section::ROM0 => self.read_rom0(address_space::ROM0::new(adr)),
            Section::ROM1 => self.read_rom1(address_space::ROM1::new(adr)),
            Section::VRAM => self.read_vram(address_space::VRAM::new(adr)),
            Section::ERAM => self.read_eram(address_space::ERAM::new(adr)),
            Section::WRAM0 => self.read_wram0(address_space::WRAM0::new(adr)),
            Section::WRAM1 => self.read_wram1(address_space::WRAM1::new(adr)),
            Section::MIRROR => self.read_mirror(address_space::MIRROR::new(adr)),
            Section::OAM => self.read_oam(address_space::OAM::new(adr)),
            Section::Unusable => self.read_unusable(address_space::Unusable::new(adr)),
            Section::IO => self.read_io(IOReg::from_index((adr & 0xFF) as u8)),
            Section::HRAM => self.read_hram(address_space::HRAM::new(adr)),
            Section::Invalid => logger::fatal!("read to invalid memory address {adr:04X}"),
        }
    }

    #[inline]
    fn read_rom0(&mut self, offset: address_space::ROM0) -> u8 {
        self.rom0[offset.get()]
    }

    #[inline]
    fn read_rom1(&self, offset: address_space::ROM1) -> u8 {
        self.rom1[offset.get()]
    }

    #[inline]
    fn read_vram(&self, offset: address_space::VRAM) -> u8 {
        PPU::read_vram(self, offset)
    }

    #[inline]
    fn read_eram(&self, offset: address_space::ERAM) -> u8 {
        self.eram[offset.get()]
    }

    #[inline]
    fn read_wram0(&self, offset: address_space::WRAM0) -> u8 {
        self.wram0[offset.get()]
    }

    #[inline]
    fn read_wram1(&self, offset: address_space::WRAM1) -> u8 {
        self.wram1[offset.get()]
    }

    #[inline]
    fn read_mirror(&self, _offset: address_space::MIRROR) -> u8 {
        todo!()
    }

    #[inline]
    fn read_oam(&self, offset: address_space::OAM) -> u8 {
        PPU::read_oam(self, offset)
    }

    #[inline]
    fn read_unusable(&self, offset: address_space::Unusable) -> u8 {
        logger::warning!(
            "Attempting to read from unusable section 0x{:04X}",
            offset.full_adr()
        );
        0xFF
    }

    #[inline]
    fn read_hram(&self, offset: address_space::HRAM) -> u8 {
        self.hram[offset.get()]
    }
}
