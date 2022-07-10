pub struct Resources {}

impl Resources {
    pub fn new(_: &mut imgui::WGPUContext) -> Self {
        Self {}
    }

    pub fn release(&mut self, _: &mut imgui::WGPUContext) {}
}
