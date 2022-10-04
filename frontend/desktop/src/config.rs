use std::path::{Path, PathBuf};

/// Global configurations.
pub struct GlobalConfig {
    pub emu: EmuConfig,
    pub frontend: FrontendConfig,
}

/// Reads the global config from disk.
pub fn global_config() -> GlobalConfig {
    load_config(&PathBuf::from("/"))
}

pub fn load_config(_path: &Path) -> GlobalConfig {
    GlobalConfig {
        emu: EmuConfig {},
        frontend: FrontendConfig {},
    }
}

/// Emulator configurations.
pub struct EmuConfig {}

/// Frontend Configurations.
pub struct FrontendConfig {}
