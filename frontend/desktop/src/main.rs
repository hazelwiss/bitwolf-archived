use std::path::PathBuf;

mod common;
mod cores;
mod ui;
mod window_loop;

enum Core {
    None,
    Core(Box<dyn common::CoreFrontend>),
}

struct Ctx {
    previously_loaded_files: Vec<PathBuf>,
    config_window_active: bool,
    help_window_active: bool,
    #[cfg(feature = "logging")]
    logger: util::Logger,
}

fn main() {
    let mut core = Core::None;
    let mut ctx = Ctx {
        previously_loaded_files: vec![],
        config_window_active: false,
        help_window_active: false,
        #[cfg(feature = "logging")]
        logger: util::Logger::new(),
    };
    window_loop::run(move |mut imgui_ctx| {
        ui::main_menu(&mut imgui_ctx, &mut ctx, &mut core);
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
