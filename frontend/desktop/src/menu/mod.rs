mod file;
mod help;
mod options;

use common_frontend::FrontendBox;
use imgui::DrawContext;

pub fn menu(draw_ctx: &mut DrawContext, frontend: &mut FrontendBox) {
    draw_ctx.ui().main_menu_bar(|| {
        file::menu(draw_ctx, frontend);
        frontend.emulation(draw_ctx);
        options::menu(draw_ctx, frontend);
        frontend.debug(draw_ctx);
        help::menu(draw_ctx, frontend);
    });
}
