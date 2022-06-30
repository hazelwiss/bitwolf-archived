use crate::GBC;
use common_frontend::constraints::destroy::Destroy;

impl Destroy for GBC {
    fn destroy(&mut self, wgpu_ctx: &mut imgui::WGPUContext) {
        wgpu_ctx.destroy_texture(self.display_texture);
    }
}
