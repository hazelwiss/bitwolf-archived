mod windows;

use crate::GBC;
use common_frontend::constraints::draw::Draw;

impl Draw for GBC {
    fn draw(&mut self, draw_ctx: &mut imgui::DrawContext) {
        windows::display::draw(self, draw_ctx);
        windows::disassembly::draw(self, draw_ctx);
    }
}
