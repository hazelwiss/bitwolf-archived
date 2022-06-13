mod menu;

use crate::{menubar::MenuBar, Frontend};

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Frontend for EmptyFrontend {
    fn menu(&mut self, draw_ctx: &mut imgui::DrawContext) {
        draw_ctx.ui().main_menu_bar(|| {
            draw_ctx.ui().menu("File", || self.file(draw_ctx));
            draw_ctx.ui().menu("Options", || self.options(draw_ctx));
            draw_ctx.ui().menu("Help", || self.help(draw_ctx));
        });
    }
}
