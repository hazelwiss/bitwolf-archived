use crate::bus::{address_space, memory_map::Section, Bus};

pub fn read(bus: &Bus, adr: u16) -> u8 {
    let section = Section::from_adr(adr);
    match section {
        Section::ROM0 => read_rom0(bus, address_space::ROM0::new(adr)),
        Section::ROM1 => read_rom1(bus, address_space::ROM1::new(adr)),
        Section::VRAM => {
            crate::ppu::debug::read::read_vram(&bus.ppu, address_space::VRAM::new(adr))
        }
        Section::ERAM => read_eram(bus, address_space::ERAM::new(adr)),
        Section::WRAM0 => read_wram0(bus, address_space::WRAM0::new(adr)),
        Section::WRAM1 => read_wram1(bus, address_space::WRAM1::new(adr)),
        Section::MIRROR => read_mirror(bus, address_space::MIRROR::new(adr)),
        Section::OAM => crate::ppu::debug::read::read_oam(&bus.ppu, address_space::OAM::new(adr)),
        Section::Unusable => logger::fatal!("debug read from unusable area."),
        Section::IO => todo!(),
        Section::HRAM => read_hram(bus, address_space::HRAM::new(adr)),
        Section::Invalid => logger::fatal!("debug read from invalid address."),
    }
}

pub fn read_rom0(bus: &Bus, adr: address_space::ROM0) -> u8 {
    bus.rom0[adr.get()]
}

pub fn read_rom1(bus: &Bus, adr: address_space::ROM1) -> u8 {
    bus.rom1[adr.get()]
}

pub fn read_eram(bus: &Bus, adr: address_space::ERAM) -> u8 {
    bus.eram[adr.get()]
}

pub fn read_wram0(bus: &Bus, adr: address_space::WRAM0) -> u8 {
    bus.wram0[adr.get()]
}

pub fn read_wram1(bus: &Bus, adr: address_space::WRAM1) -> u8 {
    bus.wram1[adr.get()]
}

pub fn read_mirror(_bus: &Bus, _adr: address_space::MIRROR) -> u8 {
    todo!()
}

pub fn read_hram(bus: &Bus, adr: address_space::HRAM) -> u8 {
    bus.hram[adr.get()]
}
