use common_frontend::{destroy::Destroy, draw::Draw, menubar::MenuBar, update::Update};

impl Update for EmptyFrontend {
    fn update(&mut self) {}
}

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl MenuBar for EmptyFrontend {
    fn debug(&mut self, _: &mut imgui::DrawContext) {}

    fn emulation(&mut self, _: &mut imgui::DrawContext) {}
}

impl Draw for EmptyFrontend {
    fn draw(&mut self, _: &mut imgui::DrawContext) {}
}

impl Destroy for EmptyFrontend {
    fn destroy(&mut self, _: &mut imgui::WGPUContext) {}
}

impl common_frontend::Frontend for EmptyFrontend {}
