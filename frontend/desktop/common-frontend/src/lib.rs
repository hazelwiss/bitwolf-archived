pub mod config;
pub mod menubar;
pub mod update;

mod frontendbox;

pub use frontendbox::FrontendBox;

pub trait Frontend: menubar::MenuBar + update::Update {}
