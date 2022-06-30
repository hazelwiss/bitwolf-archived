mod reg_file;

use super::{
    interpreter::{self, Interpreter},
    CtoF, MsgQ,
};
use crate::state as frontend_state;
use gbc_backend::Emu;

pub(super) fn sync(emu: &mut Emu<Interpreter>, msgq: &mut MsgQ) {
    msgq.try_send(CtoF::RegisterFile(reg_file::get(emu)));
}
