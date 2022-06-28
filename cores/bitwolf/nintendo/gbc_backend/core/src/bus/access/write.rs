use crate::bus::{address_space, io::IOReg, memory_map::Section, Bus};

impl Bus {
    pub fn write(&mut self, adr: u16, val: u8) {
        let section = Section::from_adr(adr);
        match section {
            Section::ROM0(offset) => self.write_rom0(offset as usize, val),
            Section::ROM1(offset) => self.write_rom1(offset as usize, val),
            Section::VRAM(offset) => self.write_vram(offset as usize, val),
            Section::ERAM(offset) => self.write_eram(offset as usize, val),
            Section::WRAM0(offset) => self.write_wram0(offset as usize, val),
            Section::WRAM1(offset) => self.write_wram1(offset as usize, val),
            Section::MIRROR(offset) => self.write_mirror(offset as usize, val),
            Section::OAM(offset) => self.write_oam(offset as usize, val),
            Section::Unusable(offset) => self.write_unusable(offset as usize, val),
            Section::IO(index) => self.write_io(IOReg::from_index(index), val),
            Section::HRAM(offset) => self.write_hram(offset as usize, val),
            Section::Invalid(offset) => logger::fatal!("write to invalid memory {offset:04X}"),
        }
    }

    #[inline]
    fn write_rom0(&mut self, _offset: address_space::ROM0, _val: u8) {
        todo!()
    }

    #[inline]
    fn write_rom1(&mut self, _offset: address_space::ROM1, _val: u8) {
        todo!()
    }

    #[inline]
    fn write_vram(&mut self, offset: address_space::VRAM, val: u8) {
        self.ppu.write_vram(offset, val);
    }

    #[inline]
    fn write_eram(&mut self, offset: address_space::ERAM, val: u8) {
        self.eram[offset] = val;
    }

    #[inline]
    fn write_wram0(&mut self, offset: address_space::WRAM0, val: u8) {
        self.wram0[offset] = val;
    }

    #[inline]
    fn write_wram1(&mut self, offset: address_space::WRAM1, val: u8) {
        self.wram1[offset] = val;
    }

    #[inline]
    fn write_mirror(&mut self, _offset: address_space::MIRROR, _val: u8) {
        todo!()
    }

    #[inline]
    fn write_oam(&mut self, offset: address_space::OAM, val: u8) {
        self.ppu.write_oam(offset, val)
    }

    #[inline]
    fn write_unusable(&mut self, offset: address_space::Unusable, val: u8) {
        logger::fatal!("Attempting to write value 0x{val:02X} to unusable section 0x{offset:04X}");
    }

    #[inline]
    fn write_hram(&mut self, offset: address_space::HRAM, val: u8) {
        self.hram[offset] = val;
    }
}
