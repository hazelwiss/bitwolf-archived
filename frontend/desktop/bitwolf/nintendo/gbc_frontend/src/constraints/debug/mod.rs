mod windows;

use crate::GBC;
use common_frontend::constraints::debug::Debug;

impl Debug for GBC {
    fn menu_debug(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().text("DEBUG");
    }

    fn draw_debug(&mut self, draw_ctx: &mut imgui::DrawContext) {
        windows::disassembly::draw(self, draw_ctx);
        windows::cpu_reg_view::draw(self, draw_ctx);
        windows::control_panel::draw(self, draw_ctx);
    }
}
