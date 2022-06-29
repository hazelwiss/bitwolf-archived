pub mod constraints;
pub mod subwindows;

mod frontendbox;

pub use frontendbox::FrontendBox;

pub trait Frontend:
    constraints::debuggable::Debuggable
    + constraints::emulation::Emulation
    + constraints::update::Update
    + constraints::draw::Draw
    + constraints::destroy::Destroy
{
}
