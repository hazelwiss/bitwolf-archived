use crate::GBC;
use common_frontend::constraints::debuggable::Debuggable;

impl Debuggable for GBC {
    fn menu_debug(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().text("DEBUG");
    }
}
