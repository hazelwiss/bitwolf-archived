pub mod config;
pub mod empty;
pub mod menubar;

use imgui::DrawContext;

pub trait Frontend: menubar::MenuBar {
    fn menu(&mut self, draw_ctx: &mut DrawContext) {
        draw_ctx.ui().main_menu_bar(|| {
            draw_ctx.ui().menu("File", || self.file(draw_ctx));
            draw_ctx.ui().menu("Emulation", || self.emulation(draw_ctx));
            draw_ctx.ui().menu("Options", || self.options(draw_ctx));
            draw_ctx.ui().menu("Debug", || self.debug(draw_ctx));
            draw_ctx.ui().menu("Help", || self.help(draw_ctx));
        });
    }
}
