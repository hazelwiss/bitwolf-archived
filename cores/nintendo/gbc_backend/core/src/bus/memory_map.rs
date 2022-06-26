pub(crate) enum Section {
    ROM0(u16),
    ROM1(u16),
    VRAM(u16),
    ERAM(u16),
    WRAM0(u16),
    WRAM1(u16),
    MIRROR(u16),
    OAM(u16),
    Unusable(u16),
    IO(u8),
    HRAM(u16),
    Invalid(u16),
}

impl Section {
    pub fn from_adr(adr: u16) -> Self {
        let section = adr >> 12;
        let offset = adr & ((1 << 12) - 1);
        match section & 0xF {
            0x0 => Self::ROM0(offset),
            0x1 => Self::ROM0(offset + 0x1000),
            0x2 => Self::ROM0(offset + 0x2000),
            0x3 => Self::ROM0(offset + 0x3000),
            0x4 => Self::ROM1(offset),
            0x5 => Self::ROM1(offset + 0x1000),
            0x6 => Self::ROM1(offset + 0x2000),
            0x7 => Self::ROM1(offset + 0x3000),
            0x8 => Self::VRAM(offset),
            0x9 => Self::VRAM(offset + 0x1000),
            0xA => Self::ERAM(offset),
            0xB => Self::ERAM(offset + 0x1000),
            0xC => Self::WRAM0(offset),
            0xD => Self::WRAM1(offset),
            _ => match adr {
                0xE000..=0xFDFF => Self::MIRROR(adr - 0xE000),
                0xFE00..=0xFE9F => Self::OAM(adr - 0xFE00),
                0xFEA0..=0xFEFF => Self::Unusable(adr - 0xFEA0),
                0xFF00..=0xFF7E => Self::IO(adr as u8),
                0xFF80..=0xFFFE => Self::HRAM(adr - 0xFF80),
                0xFFFF => Self::IO(0xFF),
                _ => Self::Invalid(adr),
            },
        }
    }
}
