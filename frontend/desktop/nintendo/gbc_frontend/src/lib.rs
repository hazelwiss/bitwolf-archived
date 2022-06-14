mod menu;

use common_frontend::Frontend;
use gbc_backend::{engines, Core};
use std::{fmt::Display, path::Path};

#[derive(Debug)]
pub enum Error {
    UnableToReadFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl std::error::Error for Error {}

enum Engine {
    Interpreter(Core<engines::interpreter::Interpreter>),
    _JIT(Core<engines::jit::JIT>),
}

pub struct GBC {
    backend: Engine,
}

impl GBC {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let rom = std::fs::read(path).or_else(|_| Err(Error::UnableToReadFile))?;
        let backend = Engine::Interpreter(Core::<engines::interpreter::Interpreter>::new(
            engines::interpreter::Builder {
                rom,
                bootrom: [0; 256],
            },
        ));
        Ok(Self { backend })
    }
}

impl Frontend for GBC {}
