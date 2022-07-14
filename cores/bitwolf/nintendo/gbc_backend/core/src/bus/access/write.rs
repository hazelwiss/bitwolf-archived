use crate::{
    bus::{address_space, io::IOReg, memory_map::Section, Bus},
    ppu::PPU,
};

impl Bus {
    pub fn write(&mut self, adr: u16, val: u8) {
        let section = Section::from_adr(adr);
        match section {
            Section::ROM0 => self.write_rom0(address_space::ROM0::new(adr), val),
            Section::ROM1 => self.write_rom1(address_space::ROM1::new(adr), val),
            Section::VRAM => self.write_vram(address_space::VRAM::new(adr), val),
            Section::ERAM => self.write_eram(address_space::ERAM::new(adr), val),
            Section::WRAM0 => self.write_wram0(address_space::WRAM0::new(adr), val),
            Section::WRAM1 => self.write_wram1(address_space::WRAM1::new(adr), val),
            Section::MIRROR => self.write_mirror(address_space::MIRROR::new(adr), val),
            Section::OAM => self.write_oam(address_space::OAM::new(adr), val),
            Section::Unusable => self.write_unusable(address_space::Unusable::new(adr), val),
            Section::IO => self.write_io(IOReg::from_index((adr & 0xFF) as u8), val),
            Section::HRAM => self.write_hram(address_space::HRAM::new(adr), val),
            Section::Invalid => logger::fatal!("write to invalid memory address {adr:04X}"),
        }
    }

    #[inline]
    fn write_rom0(&mut self, _offset: address_space::ROM0, _val: u8) {
        //todo!()
    }

    #[inline]
    fn write_rom1(&mut self, _offset: address_space::ROM1, _val: u8) {
        //todo!()
    }

    #[inline]
    fn write_vram(&mut self, offset: address_space::VRAM, val: u8) {
        PPU::write_vram(self, offset, val);
    }

    #[inline]
    fn write_eram(&mut self, offset: address_space::ERAM, val: u8) {
        self.eram[offset.get()] = val;
    }

    #[inline]
    fn write_wram0(&mut self, offset: address_space::WRAM0, val: u8) {
        self.wram0[offset.get()] = val;
    }

    #[inline]
    fn write_wram1(&mut self, offset: address_space::WRAM1, val: u8) {
        self.wram1[offset.get()] = val;
    }

    #[inline]
    fn write_mirror(&mut self, _offset: address_space::MIRROR, _val: u8) {
        todo!()
    }

    #[inline]
    fn write_oam(&mut self, offset: address_space::OAM, val: u8) {
        PPU::write_oam(self, offset, val);
    }

    #[inline]
    fn write_unusable(&mut self, offset: address_space::Unusable, val: u8) {
        logger::warning!(
            "Attempting to write value 0x{val:02X} to unusable section 0x{:04X}",
            offset.full_adr()
        );
    }

    #[inline]
    fn write_hram(&mut self, offset: address_space::HRAM, val: u8) {
        self.hram[offset.get()] = val;
    }
}
