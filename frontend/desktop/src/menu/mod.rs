mod file;
mod help;
mod options;

use common_frontend::FrontendBox;
use imgui::DrawContext;

pub fn menu(
    draw_ctx: &mut DrawContext,
    file_reader: &mut file_reader::FileReader<crate::backend_types::Types>,
    frontend: &mut FrontendBox,
) {
    draw_ctx.ui().main_menu_bar(|| {
        draw_ctx
            .ui()
            .menu("File", || file::menu(draw_ctx, file_reader, frontend));
        if frontend.emulatable() {
            draw_ctx
                .ui()
                .menu("Emulation", || frontend.menu_emulation(draw_ctx));
        }
        if frontend.debuggable() {
            draw_ctx
                .ui()
                .menu("Debug", || frontend.menu_debug(draw_ctx));
        }
        draw_ctx
            .ui()
            .menu("Options", || options::menu(draw_ctx, frontend));
        draw_ctx
            .ui()
            .menu("Help", || help::menu(draw_ctx, frontend));
    });
}
