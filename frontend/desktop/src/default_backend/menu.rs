use super::{EmptyFrontend, MenuBar};

impl MenuBar for EmptyFrontend {
    fn debug(&mut self, _: &mut imgui::DrawContext) {}

    fn emulation(&mut self, _: &mut imgui::DrawContext) {}

    fn show_debug_menu(&self) -> bool {
        false
    }

    fn show_emulation_window(&self) -> bool {
        false
    }
}
