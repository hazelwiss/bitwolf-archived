use imgui::DrawContext;

pub trait MenuBar {
    fn debug(&mut self, draw_ctx: &mut DrawContext);

    fn emulation(&mut self, draw_ctx: &mut DrawContext);
}
