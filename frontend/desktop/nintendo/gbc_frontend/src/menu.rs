use crate::GBC;
use common_frontend::menubar::MenuBar;

impl MenuBar for GBC {
    fn debug(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().text("TEST! 1");
    }

    fn emulation(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().text("TEST! 2");
    }
}
