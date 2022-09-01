#![allow(unused)]

mod cores;
mod ui;

use cores::{Core, CoreType};
use std::{fmt::Display, path::PathBuf};
use util::Logger;

pub struct Ctx {
    previously_loaded_files: Vec<(PathBuf, CoreType)>,
    config_window_active: bool,
    help_window_active: bool,
    #[cfg(feature = "log")]
    logger: Logger,
    fullscreen: bool,
}

fn main() {
    let mut core = Core::None;
    let mut ctx = Ctx {
        previously_loaded_files: vec![],
        config_window_active: false,
        help_window_active: false,
        #[cfg(feature = "log")]
        logger: Logger::default(),
        fullscreen: false,
    };
    gui::window_loop::run(move |mut gui| {
        let mut spawn_core = None;
        ui::main_menu(&mut gui, &mut ctx, &mut core, &mut spawn_core);
        {
            let ui = gui.ui();

            // Draw config subwindow.
            if ctx.config_window_active {
                gui::imgui::Window::new("Configurations")
                    .opened(&mut ctx.config_window_active)
                    .build(ui, || {
                        ui.text("Todo!");
                    });
            }

            // Draw help subwindow.
            if ctx.help_window_active {
                gui::imgui::Window::new("Help")
                    .opened(&mut ctx.help_window_active)
                    .build(ui, || {
                        ui.text("Todo!");
                    });
            }
        }

        if let Some((rom, ct)) = spawn_core {
            fn b<T>(val: T) -> Box<T> {
                Box::new(val)
            }
            let logger = &ctx.logger;
            core = Core::Core(match ct {
                CoreType::Nds => b(nds::new(&mut gui, rom, logger)),
            });
        }

        match &mut core {
            Core::Core(core) => {
                core.sync_core();
                ui::ui(&mut gui, &mut ctx, core)
            }
            Core::None => {}
        }
    });
}
