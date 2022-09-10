#[cfg(feature = "nds-core")]
pub mod nds;

use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum CoreType {
    Nds,
}

impl Display for CoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CoreType::Nds => "Nintendo DS (NDS)",
        })
    }
}

pub enum Core {
    None,
    Core(Box<dyn crate::common::CoreFrontend>),
}

impl Core {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn is_core(&self) -> bool {
        !self.is_none()
    }
}
