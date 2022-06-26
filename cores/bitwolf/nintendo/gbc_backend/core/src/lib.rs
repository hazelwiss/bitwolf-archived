#![feature(mixed_integer_ops)]

pub mod bus;
pub mod cpu;
pub mod cycles;
pub mod emu;
pub mod engines;
pub mod ppu;

mod binder;
mod events;

use common_core::framebuffer;
use engines::Engine;

type FrameBuffer = framebuffer::AccessW<framebuffer::textures::TextBGRA<160, 144>>;

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
    pub fb: FrameBuffer,
}

pub struct Emu<E: Engine> {
    _data: E::EngineData,
    cpu: cpu::CPU,
    bus: bus::Bus,
    fb: FrameBuffer,
}

impl<E: Engine> Emu<E> {
    pub fn new(builder: Builder) -> Self {
        Self {
            _data: E::EngineData::default(),
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(builder.bootrom, builder.rom),
            fb: builder.fb,
        }
    }
}
