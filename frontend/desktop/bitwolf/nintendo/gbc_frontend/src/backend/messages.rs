use super::{state::State, MsgQ};
use gbc_backend::{engines::interpreter::Interpreter, Emu};

pub(super) fn msgq_recv(emu: &mut Emu<Interpreter>, state: &mut State, msgq: &mut MsgQ) {
    while let Some(msg) = msgq.try_recv() {}
}
