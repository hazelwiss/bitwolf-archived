#![allow(unused)]
use crate::core::bus::{
    memory_map::{IOReg, Section},
    Bus,
};

impl Bus {
    pub fn read(&mut self, adr: u16) -> u8 {
        let section = Section::from_adr(adr);
        match section {
            Section::ROM0(offset) => self.read_rom0(offset),
            Section::ROM1(offset) => self.read_rom1(offset),
            Section::VRAM(offset) => self.read_vram(offset),
            Section::ERAM(offset) => self.read_eram(offset),
            Section::WRAM0(offset) => self.read_wram0(offset),
            Section::WRAM1(offset) => self.read_wram1(offset),
            Section::MIRROR(offset) => self.read_mirror(offset),
            Section::OAM(offset) => self.read_oam(offset),
            Section::Unusable(offset) => self.read_unusable(offset),
            Section::IO(reg) => self.read_io(reg),
            Section::HRAM(offset) => self.read_hram(offset),
            Section::Invalid(offset) => logger::fatal!("read to invalid memory {offset:02X}"),
        }
    }

    #[inline]
    fn read_rom0(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_rom1(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_vram(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_eram(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_wram0(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_wram1(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_mirror(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_oam(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_unusable(&self, _offset: u16) -> u8 {
        0
    }

    #[inline]
    fn read_io(&self, _reg: IOReg) -> u8 {
        0
    }

    #[inline]
    fn read_hram(&self, _offset: u16) -> u8 {
        0
    }
}
