use super::CState;
use nds_core::interpreter::Interpreter;
use std::sync::atomic::Ordering;

pub(super) fn run(mut state: CState, mut core: nds_core::core::Core<Interpreter>) {
    //let mut debug_state = nds_core::debug::rominfo::cartridge_header(&core);
    while state.running.load(Ordering::Relaxed) {
        if let Some(msg) = state.msgq.recv() {
            match msg {}
        }
        nds_core::interpreter::run_until_frame(&mut core);
        //state.dbgview_msg(DebugViewMsg::Arm9Disasm(()));
        //state.dbgview_msg(DebugViewMsg::Arm9State(()));
        //state.dbgview_msg(DebugViewMsg::Metadata(()));
    }
}
