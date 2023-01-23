use crate::GBC;
use common_frontend::constraints::video::{Video, VideoData};
use imgui::gui::TextureId;
use util::colour::BGRA;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
type Colour = BGRA;

impl Video for GBC {
    fn video_data(&self) -> VideoData {
        VideoData {
            data: unsafe {
                util::memory::to_byte_slice(self.com.fb_reader.get().read().data.as_slice())
            },
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn new_imgui_texture(&self, wgpu_ctx: &mut imgui::WGPUContext) -> TextureId {
        wgpu_ctx.create_texture([[Colour::BLACK; WIDTH]; HEIGHT])
    }
}
