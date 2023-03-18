//TMP
#![allow(dead_code)]
//
#![feature(stmt_expr_attributes)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(int_roundings)]

#[macro_use]
extern crate log;

pub mod cpu;
pub mod interpreter;

mod bus;
mod debug;

pub use interpreter::Interpreter;

use cpu::arm9::ARM9;

pub type NDSInterp = Core<Interpreter>;

pub trait Engine {
    type GlobalData: Default;
    type ARM9Data: Default;
    type ARM7Data: Default;
}

pub struct Core<E: Engine> {
    global_data: E::GlobalData,
    arm9: ARM9<E>,
}

impl<E: Engine> Core<E> {
    pub fn new() -> Self {
        let mut arm9 = ARM9::<E>::new();
        arm9.init();
        Self {
            global_data: Default::default(),
            arm9,
        }
    }
}
