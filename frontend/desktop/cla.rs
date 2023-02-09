use crate::core::Backend;
use argh::FromArgs;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

pub struct LoadRom {
    pub rom: PathBuf,
    pub backend: Backend,
}

pub enum LoadRomError {
    MissingColon(String),
    InvalidBackend(String),
    UnableToFindRom(String),
    InvalidPath(String),
}

impl Display for LoadRomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::InvalidPath(path) => {
                format!("unable to create a path out of input string '{path}'")
            }
            Self::MissingColon(str) => format!("missing colon in argument '{str}'"),
            Self::InvalidBackend(backend) => format!("use of invalid backend '{backend}'"),
            Self::UnableToFindRom(rom) => format!("unable to find rom '{rom}'"),
        })
    }
}

impl FromStr for LoadRom {
    type Err = LoadRomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (backend, rom) = s
            .split_once(':')
            .ok_or(LoadRomError::MissingColon(s.to_string()))?;
        let backend = match backend {
            "nds" => Backend::NDS,
            _ => return Err(LoadRomError::InvalidBackend(backend.to_string())),
        };
        let path = std::path::PathBuf::from_str(rom)
            .or(Err(LoadRomError::InvalidPath(rom.to_string())))?;
        if !path.exists() {
            return Err(LoadRomError::UnableToFindRom(rom.to_string()));
        }
        Ok(Self { rom: path, backend })
    }
}

/// bitwolf
#[derive(FromArgs)]
pub struct CLA {
    /// initial rom loading.
    /// [backend]:[rom]
    #[argh(option)]
    pub load_rom: Option<LoadRom>,
}

impl CLA {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

pub fn from_env() -> CLA {
    CLA::from_env()
}
