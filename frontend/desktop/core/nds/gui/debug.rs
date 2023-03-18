pub mod disasm;

use crate::core::nds::NDS;
use crate::state::ProgramState;

pub fn draw(state: &mut ProgramState<NDS>) {
    disasm::draw(state);
}
