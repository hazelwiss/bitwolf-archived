mod arm9;

use crate::{Core, Engine};

pub struct Interpreter;

impl Engine for Interpreter {
    type GlobalData = ();
    type ARM9Data = ();
    type ARM7Data = ();
}

impl lib::Core for Core<Interpreter> {
    fn run_for_cycles(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.step()
        }
    }

    fn register_info(&self) -> libfrontend::RegisterInfo {
        lib::RegisterInfo::NDS {
            arm9_gpr: self.arm9.gpr,
        }
    }

    fn step(&mut self) {}
}
