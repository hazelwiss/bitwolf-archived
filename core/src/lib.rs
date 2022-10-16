#![no_std]
#![feature(int_roundings)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
// --- Temporary ---
#![allow(dead_code)]
// -----------------

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;
#[allow(unused_imports)]
#[macro_use]
extern crate util;
extern crate self as bitwolf_core;

pub mod core;
pub mod debug;

mod common;
mod hw;

pub use crate::core::{Core, CoreBuilder};
