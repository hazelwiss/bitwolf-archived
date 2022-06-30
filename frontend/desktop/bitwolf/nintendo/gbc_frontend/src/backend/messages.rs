use super::{state::State, MsgQ};
use gbc_backend::{engines::interpreter::Interpreter, Emu};

use crate::state as frontend_state;

pub enum CtoF {
    RegisterFile(frontend_state::RegisterFile),
}

pub(super) fn msgq_recv(emu: &mut Emu<Interpreter>, state: &mut State, msgq: &mut MsgQ) {
    while let Some(msg) = msgq.try_recv() {
        match msg {}
    }
}

