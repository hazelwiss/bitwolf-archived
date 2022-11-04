#![no_std]
#![feature(int_roundings)]
#![allow(incomplete_features)]
#![allow(clippy::comparison_chain)]
#![feature(adt_const_params)]
#![feature(stmt_expr_attributes)]
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

pub use crate::core::{
    interpreter::{self, Interpreter},
    Core, CoreBuilder,
};
