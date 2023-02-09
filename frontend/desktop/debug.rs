use crossbeam::{Receiver, Sender};
use std::sync::Arc;

/// Core to Frontend
enum CtoF {}

/// Frontend to Core
enum FtoC {
    Step(usize),
}

pub struct DebugCoreInterface {
    pub steps: usize,
    receiver: Receiver<FtoC>,
    sender: Sender<CtoF>,
}

pub struct DebugInterface {
    pub reg_info: lib::RegisterInfo,
    receiver: Receiver<CtoF>,
    sender: Sender<FtoC>,
}

unsafe impl Send for DebugCoreInterface {}
unsafe impl Send for DebugInterface {}

pub fn new() -> (DebugInterface, DebugCoreInterface) {
    let bounds = 100;
    let (s0, r0) = crossbeam::bounded(bounds);
    let (s1, r1) = crossbeam::bounded(bounds);
    (
        DebugInterface {
            receiver: r1,
            sender: s0,
            reg_info: lib::RegisterInfo::NDS { arm9_gpr: [0; 16] },
        },
        DebugCoreInterface {
            receiver: r0,
            sender: s1,
            steps: 0,
        },
    )
}
