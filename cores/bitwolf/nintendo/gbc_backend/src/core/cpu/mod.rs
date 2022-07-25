pub mod instructions;
pub mod interrupt;
pub mod registers;

mod events;

use crate::{core::bus::Bus, interfaces, Engine};
use std::marker::PhantomData;

pub struct CPU<E: Engine> {
    pub(crate) bus: Bus,
    reg_file: registers::RegisterFile,
    ime: bool,
    halted: bool,
    _pd: PhantomData<E>,
}

impl<E: Engine> CPU<E> {
    pub fn new(
        bootrom: [u8; 256],
        rom: Vec<u8>,
        audio_interface: interfaces::AudioInterface,
        video_interface: interfaces::VideoInterface,
        input_interface: interfaces::InputInterface,
    ) -> Self {
        Self {
            reg_file: registers::RegisterFile::new(),
            ime: false,
            halted: false,
            bus: Bus::new(
                bootrom,
                rom,
                audio_interface,
                video_interface,
                input_interface,
            ),
            _pd: PhantomData::default(),
        }
    }

    #[inline]
    pub fn ime_set(&mut self, val: bool) {
        self.ime = val;
    }

    #[inline]
    pub fn ime_get(&self) -> bool {
        self.ime
    }

    #[inline]
    pub fn halted_get(&self) -> bool {
        self.halted
    }

    #[inline]
    pub fn halted_set(&mut self, val: bool) {
        self.halted = val;
    }
}
