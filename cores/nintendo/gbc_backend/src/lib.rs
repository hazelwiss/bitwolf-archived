pub mod engines;

use std::ops::{Deref, DerefMut};

pub struct Core<E: engines::Engine> {
    engine: E,
}

impl<E: engines::Engine> Deref for Core<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl<E: engines::Engine> DerefMut for Core<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}
