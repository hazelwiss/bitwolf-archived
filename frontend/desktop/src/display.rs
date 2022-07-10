use common_frontend::{constraints::video::VideoData, FrontendBox};
use imgui::DrawContext;

pub fn window(frontend: &mut FrontendBox, draw_ctx: &mut DrawContext) {
    if frontend.has_video() {
        let VideoData {
            data,
            width,
            height,
        } = frontend.get_inner().video_data();
        let texture = frontend.video_texture_id();
        draw_ctx
            .resources()
            .update_texture(texture, data, width as u32, height as u32);
        let ui = draw_ctx.ui();
        imgui::gui::Window::new("Display").build(ui, || {
            imgui::gui::Image::new(texture, [width as f32, height as f32])
                .size(ui.content_region_avail())
                .build(ui);
        });
    }
}

pub fn full(frontend: &mut FrontendBox, draw_ctx: &mut DrawContext) {
    if frontend.has_video() {
        let VideoData {
            data,
            width,
            height,
        } = frontend.get_inner().video_data();
        let texture = frontend.video_texture_id();
        draw_ctx
            .resources()
            .update_texture(texture, data, width as u32, height as u32);
        let ui = draw_ctx.ui();
        let p_min = [0f32, 0f32];
        let p_max = ui.io().display_size;
        ui.get_background_draw_list()
            .add_image(texture, p_min, p_max)
            .build();
    }
}
