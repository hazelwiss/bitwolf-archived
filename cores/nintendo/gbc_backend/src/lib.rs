#![feature(mixed_integer_ops)]

pub mod engines;

pub struct Core<E: engines::Engine> {
    engine: E,
}
