use crate::{Frontend, FrontendWrapper};
use imgui::{gui::TextureId, WGPUContext};

pub struct FrontendBox {
    inner: Box<dyn Frontend>,
    video_texture: TextureId,
    debugging: bool,
    debug_submenu: bool,
    emulation_submenu: bool,
    video: bool,
    fullscreen: bool,
}

impl FrontendBox {
    pub fn new(wrapper: FrontendWrapper, wgpu_ctx: &mut WGPUContext) -> Self {
        let video_texture = wrapper.frontend.new_imgui_texture(wgpu_ctx);
        Self {
            inner: wrapper.frontend,
            video_texture,
            debugging: false,
            fullscreen: true,
            debug_submenu: wrapper.has_debug_submenu,
            emulation_submenu: wrapper.has_emulation_submenu,
            video: wrapper.has_video,
        }
    }

    pub fn swap(&mut self, other: FrontendWrapper, wgpu_ctx: &mut WGPUContext) {
        self.inner.destroy(wgpu_ctx);
        wgpu_ctx.destroy_texture(self.video_texture);
        *self = Self::new(other, wgpu_ctx);
    }

    pub fn video_texture_id(&self) -> TextureId {
        self.video_texture
    }

    pub fn is_debugging(&self) -> bool {
        self.debugging
    }

    pub fn set_debugging(&mut self, val: bool) {
        self.debugging = val;
    }

    pub fn has_debug_submenu(&self) -> bool {
        self.debug_submenu
    }

    pub fn has_emulation_submenu(&self) -> bool {
        self.emulation_submenu
    }

    pub fn has_video(&self) -> bool {
        self.video
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn get_inner(&self) -> &dyn Frontend {
        self.inner.as_ref()
    }

    pub fn get_inner_mut(&mut self) -> &mut dyn Frontend {
        self.inner.as_mut()
    }
}
