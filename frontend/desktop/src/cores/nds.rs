use std::path::Path;

use util::Logger;

use crate::common::CoreFrontend;

pub struct NDS {
    nds_core: nds_core::Core<nds_core::engine::Interpreter>,
}

impl NDS {
    pub fn new(rom: Vec<u8>, logger: &Logger) -> Self {
        #[cfg(feature = "logging")]
        print_rom_info(&rom, logger);
        Self {
            nds_core: nds_core::Core::new(nds_core::Builder {
                rom,
                logger: logger.clone(),
            }),
        }
    }
}

impl CoreFrontend for NDS {
    #[cfg(debug_assertions)]
    fn update_panels(&mut self, run_ctx: &mut crate::window_loop::ImguiCtx) {}

    fn sync_core(&mut self) {
        
    }
}

#[cfg(feature = "logging")]
fn print_rom_info(rom: &Vec<u8>, logger: &Logger) {
    logger.info(format!(
        "ROM metadata.
        \rROM size: {} B ({:.3} KiB) ({:.3} MiB)  
        \rROM header:
        \rROM name: \
        ",
        rom.len(),
        (rom.len() as f32) / 1024.0,
        (rom.len() as f32 / (1024.0 * 1024.0))
    ));
}
