use imgui::DrawContext;

pub trait MenuBar {
    fn debug(&mut self, draw_ctx: &mut DrawContext);

    fn emulation(&mut self, draw_ctx: &mut DrawContext);

    fn file(&mut self, _draw_ctx: &mut DrawContext) {}

    fn options(&mut self, _draw_ctx: &mut DrawContext) {}

    fn help(&mut self, _draw_ctx: &mut DrawContext) {}
}
