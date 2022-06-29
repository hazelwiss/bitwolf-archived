use imgui::DrawContext;

pub trait Draw {
    fn draw(&mut self, draw_ctx: &mut DrawContext);
}
