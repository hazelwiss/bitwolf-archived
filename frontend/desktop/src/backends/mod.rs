pub mod nintendo;

use imgui::DrawContext;

pub enum BackendType {
    None,
    NintendoGBC(nintendo::gbc::Context),
}

pub struct Backend {
    be: BackendType,
}

impl Backend {
    pub fn none() -> Self {
        Self {
            be: BackendType::None,
        }
    }

    pub fn get(&mut self) -> &mut BackendType {
        &mut self.be
    }

    pub fn update(&mut self) {
        match self.get() {
            BackendType::NintendoGBC(gbc) => gbc.update(),
            BackendType::None => {}
        }
    }

    pub fn swap(&mut self, _: &mut DrawContext, new: BackendType) {
        self.be = new;
    }
}
