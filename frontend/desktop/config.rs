use crate::cla::{LoadRom, CLA};

pub struct Config {
    pub load_rom: Option<LoadRom>,
}

impl Config {
    pub fn from_env() -> Self {
        Self { load_rom: None }
    }

    pub fn with_cla(mut self, cla: CLA) -> Self {
        self.load_rom = cla.load_rom;
        self
    }
}

pub fn from_env() -> Config {
    Config::from_env()
}
