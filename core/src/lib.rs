#![no_std]
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
