use super::{state::State, sync, MsgQ};
use gbc_backend::{
    engines::interpreter::{self, Interpreter},
    Emu,
};

use crate::{messages::FtoC, state as frontend_state};

pub enum CtoF {
    RegisterFile(frontend_state::substates::RegisterFile),
    Control(frontend_state::substates::Control),
    Disassembly(frontend_state::substates::Disassembly),
}

#[inline(always)]
pub(super) fn msgq_recv(emu: &mut Emu<Interpreter>, state: &mut State, msgq: &mut MsgQ) {
    while let Some(msg) = msgq.try_recv() {
        match msg {
            FtoC::SetPausedState(paused) => {
                state.ctrl.running = !paused;
            }
            FtoC::Step(count) => {
                for _ in 0..count {
                    interpreter::step(emu)
                }
            }
            FtoC::StepOver => todo!(),
        }
        sync::sync(emu, state, msgq);
    }
}
