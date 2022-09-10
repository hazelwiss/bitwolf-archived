use imgui::Ui;

use crate::{
    common::debug_view::View,
    ui::gfx::{self, GfxContext},
};

pub struct Arm9Disasm {}

impl View for Arm9Disasm {
    type MutableState = ();

    fn destroy(_: &mut GfxContext) {}

    fn window_title() -> &'static str {
        "ARM9 Disasm"
    }

    fn construct_window<T: AsRef<str>>(window: imgui::Window<'_, T>) -> imgui::Window<'_, T> {
        window
    }

    fn on_state_changed(&mut self, old_state: Self::MutableState, new_state: &Self::MutableState) {}

    fn menu_bar(&mut self, ui: &Ui, state: &Self::MutableState) {}

    fn view(&mut self, ui: &Ui, state: &Self::MutableState) {}
}
