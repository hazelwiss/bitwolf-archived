pub mod debuggable;
pub mod destroy;
pub mod draw;
pub mod emulation;
pub mod update;

mod frontendbox;

pub use frontendbox::FrontendBox;

pub trait Frontend:
    debuggable::Debuggable + emulation::Emulation + update::Update + draw::Draw + destroy::Destroy
{
}
