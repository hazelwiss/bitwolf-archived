pub mod destroy;
pub mod draw;
pub mod menubar;
pub mod update;

mod frontendbox;

pub use frontendbox::FrontendBox;

pub trait Frontend: menubar::MenuBar + update::Update + draw::Draw + destroy::Destroy {}
