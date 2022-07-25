#![feature(mixed_integer_ops)]
#![feature(let_chains)]

mod core;

pub use crate::core::{apu::AudioBuffer, engines, Builder, Emu, Texture};

use crate::core::engines::Engine;
use std::ops::{Deref, DerefMut};

pub struct Core<E: Engine> {
    engine: Emu<E>,
}

impl<E: Engine> Core<E> {
    pub fn new(builder: Builder, sampler: core::Sampler) -> Self {
        Self {
            engine: Emu::new(builder, sampler),
        }
    }
}

impl<E: Engine> Deref for Core<E> {
    type Target = Emu<E>;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl<E: Engine> DerefMut for Core<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}
