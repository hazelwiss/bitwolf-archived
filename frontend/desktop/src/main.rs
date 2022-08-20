#![allow(unused)]

mod common;
mod cores;
mod ui;
mod window_loop;

use std::{fmt::Display, path::PathBuf};

use util::Logger;

#[derive(Copy, Clone)]
enum CoreType {
    Nds,
}

impl Display for CoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CoreType::Nds => "Nintendo DS (NDS)",
        })
    }
}

enum Core {
    None,
    Core(Box<dyn common::CoreFrontend>),
}

impl Core {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn is_core(&self) -> bool {
        !self.is_none()
    }
}

struct Ctx {
    previously_loaded_files: Vec<PathBuf>,
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
    window_loop::run(move |mut imgui_ctx| {
        ui::main_menu(&imgui_ctx, &mut ctx, &mut core);
        {
            let ui = imgui_ctx.ui();

            // Draw config subwindow.
            if ctx.config_window_active {
                imgui::Window::new("Configurations")
                    .opened(&mut ctx.config_window_active)
                    .build(ui, || {
                        ui.text("Todo!");
                    });
            }

            // Draw help subwindow.
            if ctx.help_window_active {
                imgui::Window::new("Help")
                    .opened(&mut ctx.help_window_active)
                    .build(ui, || {
                        ui.text("Todo!");
                    });
            }
        }

        match &mut core {
            Core::Core(core) => {
                core.sync_core();
                ui::ui(&mut imgui_ctx, &mut ctx, core)
            }
            Core::None => {}
        }
    });
}
