use crate::GBC;
use common_frontend::emulation::Emulation;

impl Emulation for GBC {
    fn menu_emulation(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().text("EMULATION");
    }
}
