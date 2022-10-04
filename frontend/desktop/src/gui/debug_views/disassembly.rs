use super::{DebugView, Ui, WindowGfx};

#[derive(Default)]
pub struct DVDisasm {}

impl DebugView for DVDisasm {
    fn draw(&mut self, _window: &mut WindowGfx, ui: &Ui) {
        ui.text("hello from disassembler!");
    }
}
