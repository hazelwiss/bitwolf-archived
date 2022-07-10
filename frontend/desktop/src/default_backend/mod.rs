use common_frontend::{
    constraints::{
        audio::Audio,
        debug::Debug,
        destroy::Destroy,
        emulation::Emulation,
        input::Input,
        update::Update,
        video::{Video, VideoData},
    },
    FrontendWrapper,
};

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> FrontendWrapper {
        FrontendWrapper {
            frontend: Box::new(Self {}),
            has_debug_submenu: false,
            has_emulation_submenu: false,
            has_video: false,
        }
    }
}

impl Update for EmptyFrontend {
    fn update(&mut self) {}
}

impl Destroy for EmptyFrontend {
    fn destroy(&mut self, _: &mut imgui::WGPUContext) {}
}

impl Debug for EmptyFrontend {
    fn menu_debug(&mut self, _: &mut imgui::DrawContext) {}

    fn draw_debug(&mut self, _: &mut imgui::DrawContext) {}
}

impl Emulation for EmptyFrontend {
    fn menu_emulation(&mut self, _: &mut imgui::DrawContext) {}
}

impl Input for EmptyFrontend {
    fn input(&mut self, _: imgui::Input) {}
}

impl Video for EmptyFrontend {
    fn video_data(&self) -> VideoData {
        VideoData {
            data: &[],
            width: 0,
            height: 0,
        }
    }

    fn new_imgui_texture(&self, _: &mut imgui::WGPUContext) -> imgui::gui::TextureId {
        imgui::gui::TextureId::new(0)
    }
}

impl Audio for EmptyFrontend {}

impl common_frontend::Frontend for EmptyFrontend {}
