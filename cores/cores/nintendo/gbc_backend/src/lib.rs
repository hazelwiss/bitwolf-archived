#![feature(mixed_integer_ops)]
#![feature(let_chains)]

pub mod debug;
pub mod engines;
pub mod input;
pub mod interfaces;

mod core;

pub use crate::core::Texture;
pub use engines::{interpreter::Interpreter, Engine};

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
    pub audio_interface: interfaces::AudioInterface,
    pub video_interface: interfaces::VideoInterface,
    pub input_interface: interfaces::InputInterface,
}

pub struct Core<E: Engine> {
    _data: E::EngineData,
    cpu: core::cpu::CPU<E>,
}

impl<E: Engine + 'static> Core<E> {
    pub fn new(builder: Builder) -> Self {
        Self {
            _data: E::EngineData::default(),
            cpu: core::cpu::CPU::new(
                builder.bootrom,
                builder.rom,
                builder.audio_interface,
                builder.video_interface,
                builder.input_interface,
            ),
        }
    }
}
