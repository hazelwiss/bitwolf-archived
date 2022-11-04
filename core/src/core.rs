pub mod arm7;
pub mod arm9;
pub mod bus;
pub mod cartridge;
pub mod engine;
pub mod interpreter;
pub mod registers;

use alloc::{boxed::Box, vec::Vec};
use engine::Engine;
use util::log::{self, Logger};

pub struct CoreBuilder {
    pub rom: Vec<u8>,
    pub log: Logger,
}

impl Default for CoreBuilder {
    fn default() -> Self {
        Self {
            rom: vec![],
            log: Logger::root(log::Discard, log::o!()),
        }
    }
}

impl CoreBuilder {
    pub fn build<E: Engine>(self) -> Core<E> {
        debug_assert!(self.rom.len() > 0x200);
        let header = cartridge::Header::from_rom(&self.rom);
        let arm9 = arm9::Arm9::default();
        let arm7 = arm7::Arm7 {};
        let mut main_memory = Box::new([0; mb!(4)]);
        main_memory[0x3FFE00..].copy_from_slice(&self.rom[..0x200]);
        main_memory[(header.arm9_load_adr() & (mb!(4) - 1)) as usize
            ..(header.arm9_load_adr() & (mb!(4) - 1)) as usize + header.arm9_size() as usize]
            .copy_from_slice(
                &self.rom[header.arm9_rom_adr() as usize
                    ..(header.arm9_rom_adr() + header.arm9_size()) as usize],
            );
        let mut core = Core {
            arm9,
            arm7,
            main_memory,
            cartidge_header: header,
            log: self.log,
            engine_data: Default::default(),
        };
        arm9::Arm9::reset(&mut core);
        core
    }

    pub fn rom(mut self, rom: Vec<u8>) -> Self {
        self.rom = rom;
        self
    }

    pub fn log(mut self, log: Logger) -> Self {
        self.log = log;
        self
    }
}

pub struct Core<E: Engine> {
    pub arm9: arm9::Arm9<E>,
    pub arm7: arm7::Arm7,
    engine_data: E::GlobalData,
    main_memory: Box<[u8; mb!(4)]>,
    pub(crate) cartidge_header: cartridge::Header,
    pub(crate) log: Logger,
}
