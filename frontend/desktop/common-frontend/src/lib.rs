pub mod constraints;
pub mod framebuffer;
pub mod subwindows;

mod frontendbox;

pub use frontendbox::FrontendBox;

pub struct FrontendWrapper {
    pub frontend: Box<dyn Frontend>,
    pub has_debug_submenu: bool,
    pub has_emulation_submenu: bool,
    pub has_video: bool,
}

impl FrontendWrapper {
    pub fn new(frontend: Box<impl Frontend + 'static>) -> Self {
        Self {
            frontend,
            has_debug_submenu: true,
            has_emulation_submenu: true,
            has_video: true,
        }
    }
}

pub trait Frontend:
    constraints::debug::Debug
    + constraints::emulation::Emulation
    + constraints::update::Update
    + constraints::destroy::Destroy
    + constraints::input::Input
    + constraints::video::Video
    + constraints::audio::Audio
{
}
