use crate::frontend::nds::{DebugGuiFunctional, State};
use std::sync::atomic::Ordering;

pub struct Control;

impl Default for Control {
    fn default() -> Self {
        Self
    }
}

impl DebugGuiFunctional for Control {
    fn draw(state: &mut State, ui: &imgui::Ui, io: &imgui::Io) {
        let halted = state.halted.load(Ordering::Relaxed);
        ui.text(if halted { "halted" } else { "running" });
        if ui.button(if halted { "|>" } else { "||" }) {
            state.halted.fetch_not(Ordering::Relaxed);
        }
    }
}
