use crate::GBC;

pub fn draw(gbc: &mut GBC, draw_ctx: &mut imgui::DrawContext) {
    let fb = gbc.fb.get();
    let read = fb.read();
    draw_ctx.resources().update_texture(
        gbc.resources.display_texture,
        unsafe { util::memory::to_byte_slice(&read.data) },
        160,
        144,
    );
    let ui = draw_ctx.ui();
    imgui::gui::Window::new("Display").build(ui, || {
        imgui::gui::Image::new(gbc.resources.display_texture, [160.0, 144.0])
            .size(ui.content_region_avail())
            .build(ui);
    });
}
