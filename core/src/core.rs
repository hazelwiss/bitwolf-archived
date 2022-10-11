pub mod arm7;
pub mod arm9;
pub mod cartridge;
pub mod engine;
pub mod registers;

use alloc::{boxed::Box, vec::Vec};
use util::log::{self, Logger};

pub struct CoreBuilder {
    rom: Vec<u8>,
    log: Logger,
}

impl CoreBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            rom: vec![],
            log: Logger::root(log::Discard, log::o!()),
        }
    }

    pub fn build(self) -> Core {
        debug_assert!(self.rom.len() > 0x200);
        let header = cartridge::Header::from_rom(&self.rom);
        let mut arm9 = arm9::ARM9::new();
        let arm7 = arm7::ARM7 {};
        arm9.reset(&header);
        let main_memory = Box::new([0; mb!(4)]);
        //main_memory[0x7FFE00..].copy_from_slice(&rom[..0x200]);
        Core {
            arm9,
            arm7,
            main_memory,
            cartidge_header: header,
        }
    }

    pub fn rom(mut self, rom: Vec<u8>) -> Self {
        self.rom = rom;
        self
    }
}

pub struct Core {
    pub arm9: arm9::ARM9,
    pub arm7: arm7::ARM7,
    main_memory: Box<[u8; mb!(4)]>,
    pub(crate) cartidge_header: cartridge::Header,
}

impl Core {
    pub fn read(&self, adr: u32) -> u32 {
        let adr = adr as usize - 0x02000000;
        let mut val = 0;
        val |= self.main_memory[adr] as u32;
        val |= (self.main_memory[adr + 1] as u32) << 8;
        val |= (self.main_memory[adr + 2] as u32) << 16;
        val |= (self.main_memory[adr + 3] as u32) << 24;
        val
    }
}
