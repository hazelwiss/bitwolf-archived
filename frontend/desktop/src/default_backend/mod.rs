mod menu;
mod update;

pub struct EmptyFrontend {}

impl EmptyFrontend {
    pub fn new() -> Self {
        Self {}
    }
}

impl common_frontend::Frontend for EmptyFrontend {}
