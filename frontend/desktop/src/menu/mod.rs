mod file;
mod help;
mod options;

use crate::FrontendBox;
use imgui::DrawContext;

pub fn draw(draw_ctx: &mut DrawContext, frontend: &mut FrontendBox) {
    draw_ctx.ui().main_menu_bar(|| {
        draw_ctx
            .ui()
            .menu("File", || file::draw(draw_ctx, frontend));
        if frontend.show_emulation_window() {
            draw_ctx
                .ui()
                .menu("Emulation", || frontend.emulation(draw_ctx));
        }
        if frontend.show_debug_menu() {
            draw_ctx.ui().menu("Debug", || frontend.debug(draw_ctx));
        }
        draw_ctx
            .ui()
            .menu("Options", || options::draw(draw_ctx, frontend));
        draw_ctx
            .ui()
            .menu("Help", || help::draw(draw_ctx, frontend));
    });
}
