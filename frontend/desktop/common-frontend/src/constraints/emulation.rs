use imgui::DrawContext;

pub trait Emulation {
    fn menu_emulation(&mut self, draw_ctx: &mut DrawContext);
}
