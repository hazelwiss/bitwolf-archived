use crate::GBC;
use common_frontend::draw::Draw;

impl Draw for GBC {
    fn draw(&mut self, draw_ctx: &mut imgui::DrawContext) {
        let fb = self.fb.get();
        let read = fb.read();
        draw_ctx.resources().update_texture(
            self.display_texture,
            unsafe { util::memory::to_byte_slice(&read.text) },
            160,
            144,
        );
        let ui = draw_ctx.ui();
        imgui::gui::Window::new("Display").build(ui, || {
            imgui::gui::Image::new(self.display_texture, [160.0, 144.0])
                .size(ui.content_region_avail())
                .build(ui);
        });
    }
}
