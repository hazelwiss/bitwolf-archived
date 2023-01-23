use crate::spawn::backend::Com;
use gbc_backend::{engines::interpreter, Builder, Interpreter};
use std::sync::atomic::Ordering;

pub fn run_normal(builder: Builder, com: Box<Com>) {
    let mut core = gbc_backend::Core::<Interpreter>::new(builder);
    while com.running.load(Ordering::Relaxed) {
        interpreter::step(&mut core);
    }
}
