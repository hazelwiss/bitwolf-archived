use super::{cartridge::Header, registers::RegFile};

pub struct ARM9 {
    pub registers: RegFile,
}

impl ARM9 {
    pub fn new() -> Self {
        Self {
            registers: RegFile { gpr: [0; 16] },
        }
    }

    pub fn reset(&mut self, cartridge_header: &Header) {
        self.registers.set_pc(cartridge_header.arm9_entry_address());
    }
}
