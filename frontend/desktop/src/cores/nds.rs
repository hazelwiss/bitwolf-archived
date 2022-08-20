mod core_update_loop;
#[cfg(debug_assertions)]
mod debug_state;
#[cfg(debug_assertions)]
mod update_panels;

use crate::common::CoreFrontend;
use nds_core::{
    core::{Builder, Core},
    engine::Interpreter,
};
use util::Logger;

pub struct NDSFrontend {
    core_state: core_update_loop::CoreState,
    #[cfg(debug_assertions)]
    disassembler_open: bool,
    #[cfg(debug_assertions)]
    debug_state: debug_state::DebugState,
}

impl NDSFrontend {
    pub fn new(rom: Vec<u8>, logger: &Logger) -> Self {
        #[cfg(feature = "log")]
        print_rom_info(&rom, logger);
        let core_state = core_update_loop::ThreadedBuilder {
            msgq_capacity: 256,
            core_builder: nds_core::core::Builder {
                rom,
                logger: logger.clone(),
            },
        }
        .build::<nds_core::engine::Interpreter>();
        Self {
            core_state,
            #[cfg(debug_assertions)]
            disassembler_open: true,
            #[cfg(debug_assertions)]
            debug_state: debug_state::DebugState::default(),
        }
    }
}

impl CoreFrontend for NDSFrontend {
    #[cfg(debug_assertions)]
    fn update_panels(&mut self, imgui_ctx: &mut crate::window_loop::ImguiCtx) {
        update_panels::update_panels(self, imgui_ctx);
    }

    fn sync_core(&mut self) {
        #[cfg(debug_assertions)]
        debug_state::sync_debug_state(self);
        //self.nds_core
    }
}

#[cfg(feature = "log")]
fn print_rom_info(rom: &Vec<u8>, logger: &Logger) {
    let cartridge_header = nds_core::rom::parse_rom(rom);
    logger.info(format!("Cartridge header:\n{}", unsafe {
        util::dumpable::UnsafeDumpString::dump(&cartridge_header)
    }));
}
