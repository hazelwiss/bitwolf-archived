use crate::GBC;
use common_frontend::draw::Draw;

impl Draw for GBC {
    fn draw(&mut self, draw_ctx: &mut imgui::DrawContext) {
        let ui = draw_ctx.ui();
        imgui::gui::Window::new("FB test").build(ui, || ui.text("hello!"));
    }
}
