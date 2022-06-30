#![feature(mixed_integer_ops)]

pub mod bus;
pub mod cpu;
pub mod cycles;
pub mod emu;
pub mod engines;
pub mod ppu;

mod events;

pub use ppu::lcd::{TextCol, Texture};

use engines::Engine;

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
}

pub struct Emu<E: Engine> {
    _data: E::EngineData,
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl<E: Engine> Emu<E> {
    pub fn new(builder: Builder) -> Self {
        Self {
            _data: E::EngineData::default(),
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(builder.bootrom, builder.rom),
        }
    }
}
