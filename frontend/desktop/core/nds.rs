mod gui;

use super::Core___;
use crate::state::ProgramState;

pub struct NDS {}

impl NDS {
    pub fn new() -> Self {
        Self {}
    }
}

impl Core___ for NDS {
    fn run_until_sync(state: &mut ProgramState<Self>) {}

    fn draw_debug(state: &mut ProgramState<Self>, ui: &mut imgui::Ui, io: &imgui::Io) {
        gui::debug_draw(state)
    }
}
