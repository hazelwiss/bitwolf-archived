use super::EmptyFrontend;
use crate::menubar::MenuBar;

impl MenuBar for EmptyFrontend {
    fn debug(&mut self, _: &mut imgui::DrawContext) {}

    fn emulation(&mut self, _: &mut imgui::DrawContext) {}
}
