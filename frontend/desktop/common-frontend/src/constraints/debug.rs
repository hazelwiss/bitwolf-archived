use imgui::DrawContext;

pub trait Debug {
    fn menu_debug(&mut self, draw_ctx: &mut DrawContext);

    fn draw_debug(&mut self, draw_ctx: &mut DrawContext);
}
