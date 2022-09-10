use crate::common::debug_view::View;
use imgui::Ui;

use crate::ui::gfx::GfxContext;

pub struct Arm9State {}

impl View for Arm9State {
    type MutableState = ();

    fn destroy(_: &mut GfxContext) {}

    fn window_title() -> &'static str {
        "ARM9 CPU State"
    }

    fn construct_window<T: AsRef<str>>(window: imgui::Window<'_, T>) -> imgui::Window<'_, T> {
        window
    }

    fn on_state_changed(&mut self, old_state: Self::MutableState, new_state: &Self::MutableState) {}

    fn menu_bar(&mut self, ui: &Ui, state: &Self::MutableState) {}

    fn view(&mut self, ui: &Ui, state: &Self::MutableState) {}
}
