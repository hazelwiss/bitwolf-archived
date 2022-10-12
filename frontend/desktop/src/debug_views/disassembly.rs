use crate::gui::window::Window;

use super::{DebugView, GlobalState, Ui};

#[derive(Default)]
pub struct DVDisasm;

#[derive(Debug, Default)]
pub struct State {
    pub start_adr: u32,
    pub disasm: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Default)]
pub struct Conf {
    pub start_adr: u32,
    pub line_cnt: usize,
}

impl DebugView for DVDisasm {
    type State = State;
    type Conf = Conf;

    fn draw(
        &mut self,
        _state: &mut State,
        global_state: &GlobalState,
        _window: &mut Window,
        ui: &Ui,
    ) {
        ui.text(format!(
            "hello from disassembler! pc: {:08X}",
            global_state.registers.pc()
        ));
    }

    #[inline]
    fn on_change(&mut self, _old: Self::State, _new: &mut Self::State) {}

    #[inline]
    fn config(&self, state: &Self::State) -> Option<Self::Conf> {
        Some(Conf {
            start_adr: state.start_adr,
            line_cnt: 20,
        })
    }
}
