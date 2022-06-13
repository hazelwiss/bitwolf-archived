pub mod config;
pub mod menubar;

pub trait Frontend: menubar::MenuBar {}

pub type FrontendBox = Box<dyn Frontend>;
