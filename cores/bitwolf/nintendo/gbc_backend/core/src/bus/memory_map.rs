pub(crate) enum Section {
    ROM0,
    ROM1,
    VRAM,
    ERAM,
    WRAM0,
    WRAM1,
    MIRROR,
    OAM,
    Unusable,
    IO,
    HRAM,
    Invalid,
}

impl Section {
    pub fn from_adr(adr: u16) -> Self {
        let section = adr >> 12;
        match section & 0xF {
            0x0 => Self::ROM0,
            0x1 => Self::ROM0,
            0x2 => Self::ROM0,
            0x3 => Self::ROM0,
            0x4 => Self::ROM1,
            0x5 => Self::ROM1,
            0x6 => Self::ROM1,
            0x7 => Self::ROM1,
            0x8 => Self::VRAM,
            0x9 => Self::VRAM,
            0xA => Self::ERAM,
            0xB => Self::ERAM,
            0xC => Self::WRAM0,
            0xD => Self::WRAM1,
            _ => match adr {
                0xE000..=0xFDFF => Self::MIRROR,
                0xFE00..=0xFE9F => Self::OAM,
                0xFEA0..=0xFEFF => Self::Unusable,
                0xFF00..=0xFF7E => Self::IO,
                0xFF80..=0xFFFE => Self::HRAM,
                0xFFFF => Self::IO,
                _ => Self::Invalid,
            },
        }
    }
}
