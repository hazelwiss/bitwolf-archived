mod menu;

use common_frontend::Frontend;

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Frontend for EmptyFrontend {}
