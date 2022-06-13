use imgui::DrawContext;

pub trait MenuBar {
    fn debug(&mut self, draw_ctx: &mut DrawContext);

    fn emulation(&mut self, draw_ctx: &mut DrawContext);

    fn show_debug_menu(&self) -> bool;

    fn show_emulation_window(&self) -> bool;
}
