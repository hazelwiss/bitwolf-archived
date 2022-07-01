mod ctrl;
mod disasm;
mod reg_file;

use super::{messages::CtoF, state::State, MsgQ};
use gbc_backend::{engines::interpreter::Interpreter, Emu};

pub(super) fn sync(emu: &mut Emu<Interpreter>, state: &State, msgq: &mut MsgQ) {
    msgq.try_send(CtoF::RegisterFile(reg_file::get(emu)));
    msgq.try_send(CtoF::Control(ctrl::get(state)));
    msgq.try_send(CtoF::Disassembly(disasm::get(emu)));
}
