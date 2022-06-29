use common_frontend::constraints::{
    debuggable::Debuggable, destroy::Destroy, draw::Draw, emulation::Emulation, update::Update,
};

impl Update for EmptyFrontend {
    fn update(&mut self) {}
}

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Draw for EmptyFrontend {
    fn draw(&mut self, _: &mut imgui::DrawContext) {}
}

impl Destroy for EmptyFrontend {
    fn destroy(&mut self, _: &mut imgui::WGPUContext) {}
}

impl Debuggable for EmptyFrontend {
    fn debuggable(&self) -> bool {
        false
    }

    fn menu_debug(&mut self, _: &mut imgui::DrawContext) {}
}

impl Emulation for EmptyFrontend {
    fn emulatable(&self) -> bool {
        false
    }

    fn menu_emulation(&mut self, _: &mut imgui::DrawContext) {}
}

impl common_frontend::Frontend for EmptyFrontend {}
