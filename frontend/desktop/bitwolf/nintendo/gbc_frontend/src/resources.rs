pub struct Resources {
    pub display_texture: imgui::gui::TextureId,
}

impl Resources {
    pub fn new(wgpu_ctx: &mut imgui::WGPUContext) -> Self {
        Self {
            display_texture: wgpu_ctx.create_texture([[util::colour::BGRA::WHITE; 160]; 144]),
        }
    }

    pub fn release(&mut self, wgpu_ctx: &mut imgui::WGPUContext) {
        wgpu_ctx.destroy_texture(self.display_texture);
    }
}
