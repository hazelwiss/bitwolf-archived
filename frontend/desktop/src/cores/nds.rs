mod core_loop;

#[cfg(feature = "debug-views")]
mod debug_views;

use crate::{
    common::CoreFrontend,
    ui::gfx::{self, GfxContext},
};
use imgui::Ui;
use util::Logger;

use self::core_loop::MsgCtoF;

pub struct NDSFrontend {
    fstate: core_loop::FState,
    #[cfg(feature = "debug-views")]
    debug_views: debug_views::UIState,
}

impl NDSFrontend {
    fn new(gfx: &mut GfxContext, rom: Vec<u8>, logger: &Logger) -> Self {
        #[cfg(feature = "log")]
        print_rom_info(&rom, logger);
        let debug_views_builder = {
            use debug_views::{
                arm9_disasm::Arm9Disasm, arm9_state::Arm9State, metadata::Metadata, Builder,
            };
            Builder {
                arm9_disasm: Arm9Disasm {},
                arm9_state: Arm9State {},
                metadata: Metadata::new(nds_core::rom::parse_rom(&rom).header),
            }
        };
        let core_state = core_loop::Builder {
            msgq_capacity: 256,
            core_builder: nds_core::core::Builder {
                rom,
                logger: logger.clone(),
            },
        }
        .build_interp();
        Self {
            fstate: core_state,
            #[cfg(feature = "debug-views")]
            debug_views: debug_views_builder.build(),
        }
    }
}

impl CoreFrontend for NDSFrontend {
    #[cfg(feature = "debug-views")]
    fn debug_views(&mut self, ui: &Ui) {
        self.debug_views.views(ui);
    }

    #[cfg(feature = "debug-views")]
    fn debug_views_menu(&mut self, ui: &Ui) {
        self.debug_views.debug_view_submenu(ui)
    }

    fn sync_core(&mut self) {
        while let Some(recv) = self.fstate.msgq.recv() {
            match recv {
                #[cfg(feature = "debug-views")]
                MsgCtoF::DebugView(dv) => self.debug_views.recv_message(dv),
            }
        }
    }
}

#[cfg(feature = "log")]
fn print_rom_info(rom: &Vec<u8>, logger: &Logger) {
    let cartridge = nds_core::rom::parse_rom(rom);
    logger.info(format!("Cartridge dump:\n{}", unsafe {
        util::dump_unsafe!(cartridge)
    }));
}

pub fn new(gfx: &mut GfxContext, rom: Vec<u8>, logger: &Logger) -> NDSFrontend {
    NDSFrontend::new(gfx, rom, logger)
}
