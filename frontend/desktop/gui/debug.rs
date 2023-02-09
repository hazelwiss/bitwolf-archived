mod cpu;

use imgui::Ui;

use self::cpu::CPUView;
use super::GUIState;

pub struct DebugUI {
    pub cpu_view: CPUView,
}

impl DebugUI {
    pub fn new() -> Self {
        Self {
            cpu_view: CPUView::new(),
        }
    }
}

pub(super) fn draw(state: &mut GUIState, ui: &Ui) {
    cpu::draw(state, ui);
}

pub trait Debugable {
    fn submenu(&mut self);

    fn draw(&mut self);
}
