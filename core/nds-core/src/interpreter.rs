pub mod arm9;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Interpreter;

impl crate::engine::Engine for Interpreter {
    type ARM9Data = ();
    type ARM7Data = ();
    type GlobalData = ();

    fn into_data() -> (Self::ARM9Data, Self::ARM7Data, Self::GlobalData) {
        ((), (), ())
    }
}

/// Run the core for n instructions
pub fn run_until_frame(core: &mut crate::core::Core<Interpreter>) {
    for _ in 0..512 {
        arm9::step_arm9(core);
    }
}
