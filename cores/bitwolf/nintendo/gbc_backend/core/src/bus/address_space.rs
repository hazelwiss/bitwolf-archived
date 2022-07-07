#[derive(Clone, Copy)]
pub struct AddressSpace<const RB: u16, const RE: u16>(u16);

impl<const RB: u16, const RE: u16> AddressSpace<RB, RE> {
    pub fn new(adr: u16) -> Self {
        debug_assert!(
            (RB..=RE).contains(&adr),
            "{adr:04X} is not contained within range {RB:04X} ..= {RE:04X}"
        );
        Self(adr - RB)
    }

    pub fn get(&self) -> usize {
        self.0 as usize
    }

    pub fn full_adr(&self) -> u16 {
        RB + self.0
    }
}

pub type ROM0 = AddressSpace<0x0000, 0x3FFF>;
pub type ROM1 = AddressSpace<0x4000, 0x7FFF>;
pub type VRAM = AddressSpace<0x8000, 0x9FFF>;
pub type ERAM = AddressSpace<0xA000, 0xBFFF>;
pub type WRAM0 = AddressSpace<0xC000, 0xCFFF>;
pub type WRAM1 = AddressSpace<0xD000, 0xDFFF>;
pub type MIRROR = AddressSpace<0xE000, 0xFDFF>;
pub type OAM = AddressSpace<0xFE00, 0xFE9F>;
pub type Unusable = AddressSpace<0xFEA0, 0xFEFF>;
pub type HRAM = AddressSpace<0xFF80, 0xFFFE>;
