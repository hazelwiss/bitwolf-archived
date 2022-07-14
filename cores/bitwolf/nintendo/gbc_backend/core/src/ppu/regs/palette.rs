use crate::ppu::palette::{Colour, Index};

pub(in crate::ppu) struct PaletteRegister {
    index_0: Colour,
    index_1: Colour,
    index_2: Colour,
    index_3: Colour,
}

impl PaletteRegister {
    pub fn new() -> Self {
        Self {
            index_0: Colour::C0,
            index_1: Colour::C0,
            index_2: Colour::C0,
            index_3: Colour::C0,
        }
    }

    pub fn from_u8(val: u8) -> Self {
        const COLOUR_LUT: [Colour; 4] = [Colour::C0, Colour::C1, Colour::C2, Colour::C3];
        let index_0 = COLOUR_LUT[(val & 0b11) as usize];
        let index_1 = COLOUR_LUT[((val >> 2) & 0b11) as usize];
        let index_2 = COLOUR_LUT[((val >> 4) & 0b11) as usize];
        let index_3 = COLOUR_LUT[((val >> 6) & 0b11) as usize];
        Self {
            index_0,
            index_1,
            index_2,
            index_3,
        }
    }

    pub fn as_u8(&self) -> u8 {
        fn colour_as_number(col: Colour) -> u8 {
            match col {
                Colour::C0 => 0,
                Colour::C1 => 1,
                Colour::C2 => 2,
                Colour::C3 => 3,
            }
        }
        let mut byte = 0;
        byte |= colour_as_number(self.index_0);
        byte |= colour_as_number(self.index_1) << 2;
        byte |= colour_as_number(self.index_2) << 4;
        byte |= colour_as_number(self.index_3) << 6;
        byte
    }

    pub fn get_col_from_index(&self, index: Index) -> Colour {
        match index {
            Index::I0 => self.index_0,
            Index::I1 => self.index_1,
            Index::I2 => self.index_2,
            Index::I3 => self.index_3,
        }
    }
}
