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
        draw_ctx.ui().enabled(frontend.has_emulation_submenu(), || {
            draw_ctx.ui().menu("Emulation", || {
                frontend.get_inner_mut().menu_emulation(draw_ctx)
            });
        });
        draw_ctx.ui().enabled(frontend.has_debug_submenu(), || {
            draw_ctx.ui().menu("Debug", || {
                if frontend.is_debugging() {
                    if draw_ctx.ui().button("Stop debugging") {
                        frontend.set_debugging(false);
                    }
                    frontend.get_inner_mut().menu_debug(draw_ctx)
                } else {
                    if draw_ctx.ui().button("Start debugging") {
                        frontend.set_debugging(true);
                    }
                }
            });
        });
        draw_ctx
            .ui()
            .menu("Options", || options::menu(draw_ctx, frontend));
        draw_ctx
            .ui()
            .menu("Help", || help::menu(draw_ctx, frontend));
    });
}
