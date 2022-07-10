use imgui::WGPUContext;

pub struct VideoData<'a> {
    pub data: &'a [u8],
    pub width: usize,
    pub height: usize,
}

pub trait Video {
    fn video_data(&self) -> VideoData;

    fn new_imgui_texture(&self, wgpu_ctx: &mut WGPUContext) -> imgui::gui::TextureId;
}
