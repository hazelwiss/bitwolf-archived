use imgui::WGPUContext;

pub trait Destroy {
    fn destroy(&mut self, wgpu_ctx: &mut WGPUContext);
}
