use super::SharedData;
use crate::debug::DebugCoreInterface;
use nds::Interpreter;
use std::sync::{atomic::Ordering, Arc};

pub(super) fn run_core_interp_threaded(
    mut core: nds::Core<Interpreter>,
    shared_data: Arc<SharedData>,
    mut dbg: DebugCoreInterface,
) {
    while shared_data.running.load(Ordering::Relaxed) {
        run_core_interp(
            &mut core,
            shared_data.paused.load(Ordering::Relaxed),
            &mut dbg,
        );
    }
}

pub(super) fn run_core_interp(
    core: &mut nds::Core<Interpreter>,
    paused: bool,
    dbg: &mut DebugCoreInterface,
) {
    if paused {
        while dbg.steps > 0 {
            dbg.steps -= 1;
        }
    } else {
    }
}

enum CtoF {}

enum FotC {}

pub struct Interface;

impl super::InterfaceImpl for Interface {
    type CtoF = CtoF;
    type FtoC = FotC;
}

impl super::Interface<Interface> {}

impl super::CoreInterface<Interface> {}
