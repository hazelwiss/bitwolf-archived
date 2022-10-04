mod core;

use util::log::Logger;

use crate::config::EmuConfig;
use std::{fs, path::Path};

#[allow(dead_code)]
pub struct EmuState {
    core: Option<core::CoreRunner>,
    log: Logger,
}

#[allow(dead_code)]
impl EmuState {
    pub fn new(log: Logger) -> Self {
        Self { core: None, log }
    }

    pub fn new_with_rom(log: Logger, config: &EmuConfig, rom: &Path) -> Self {
        let mut new = Self::new(log);
        new.reload(config, rom);
        new
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.log.clone());
    }

    pub fn reload(&mut self, config: &EmuConfig, rom: &Path) {
        self.kill_core();
        self.spawn_core(config, rom);
    }

    fn spawn_core(&mut self, config: &EmuConfig, rom: &Path) {
        match fs::read(rom) {
            Ok(rom) => self.core = Some(core::CoreRunner::spawn(self.log.clone(), config, rom)),
            Err(err) => panic!("{err:?}"),
        }
    }

    fn kill_core(&mut self) {
        self.core.take();
    }
}
