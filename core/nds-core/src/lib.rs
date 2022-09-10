#![no_std]
#![allow(unused)]
#![feature(inline_const)]
#![feature(const_maybe_uninit_zeroed)]

extern crate alloc;

pub mod bus;
pub mod core;
#[cfg(feature = "debug")]
pub mod debug;
pub mod engine;
pub mod interpreter;
pub mod rom;

mod cpu;
#[cfg(test)]
mod test;
