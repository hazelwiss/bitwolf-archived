mod menu;

use frontend::{menubar::MenuBar, Frontend};

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Frontend for EmptyFrontend {}
