mod debug;
mod emulation;
mod file;
mod help;
mod options;

use crate::backends::Backend;
use imgui::DrawContext;

pub fn draw(draw_ctx: &mut DrawContext, backend: &mut Backend) {
    draw_ctx.ui().main_menu_bar(|| {
        file::draw(draw_ctx, backend);
        emulation::draw(draw_ctx, backend);
        options::draw(draw_ctx, backend);
        debug::draw(draw_ctx, backend);
        help::draw(draw_ctx, backend);
    });
}
