mod frontend;

use gbc_backend::{Builder, Core, Interpreter, JIT};
use std::{fmt::Display, path::Path};

#[derive(Debug)]
pub enum Error {
    UnableToReadFile,
    UnableToReadBootrom,
    InvalidBootromSize(u64),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl std::error::Error for Error {}

enum Engine {
    Interpreter(Core<Interpreter>),
    _JIT(Core<JIT>),
}

pub struct GBC {
    backend: Engine,
}

impl GBC {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let bootrom = std::fs::read("/home/nibble/Downloads/dmg_boot.bin")
            .or_else(|_| Err(Error::UnableToReadBootrom))?;
        let rom = std::fs::read(path).or_else(|_| Err(Error::UnableToReadFile))?;
        if bootrom.len() != 256 {
            return Err(Error::InvalidBootromSize(bootrom.len() as u64));
        }
        let bootrom = {
            let mut arr = [0; 256];
            for i in 0..256 {
                arr[i] = bootrom[i];
            }
            arr
        };
        let backend = Engine::Interpreter(Core::<Interpreter>::new(Builder { rom, bootrom }));
        Ok(Self { backend })
    }
}

impl common_frontend::Frontend for GBC {}
