use std::path::PathBuf;

use imgui::StyleColor;
use rfd::FileHandle;

use crate::{common::CoreFrontend, window_loop::ImguiCtx, Core, Ctx};
use crate::{cores, CoreType};
#[cfg(feature = "nds-core")]
use cores::nds::NDSFrontend;

struct RFDFilter {
    name: &'static str,
    extensions: &'static [&'static str],
}

#[cfg(feature = "nds-core")]
const NDS_FILTER: RFDFilter = RFDFilter {
    name: "NDS (Nintendo DS)",
    extensions: &["nds"],
};

const ALL_FILTERS: &[RFDFilter] = &[
    #[cfg(feature = "nds-core")]
    NDS_FILTER,
];

pub(crate) fn main_menu(imgui_ctx: &ImguiCtx, ctx: &mut Ctx, core: &mut Core) {
    let ui = imgui_ctx.ui();
    ui.main_menu_bar(|| {
        let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
        // File submenu.
        ui.menu("File", || {
            let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
            let [pos_x, pos_y] = ui.cursor_pos();
            let width = ui.calc_item_width();
            ui.set_cursor_pos([pos_x - width * 0.015, pos_y]);
            let open_file =
                |ctx: &mut Ctx, core_type: Option<CoreType>| -> Option<(FileHandle, CoreType)> {
                    let default_fd = rfd::AsyncFileDialog::new().set_title("Load ROM");
                    let fd = if let Some(core_type) = core_type {
                        match core_type {
                            #[cfg(feature = "nds-core")]
                            CoreType::Nds => {
                                default_fd.add_filter(NDS_FILTER.name, NDS_FILTER.extensions)
                            }
                        }
                    } else {
                        let mut fd = default_fd;
                        for filter in ALL_FILTERS {
                            fd = fd.add_filter(filter.name, filter.extensions);
                        }
                        fd
                    };
                    let file = pollster::block_on(fd.pick_file());
                    if let Some(file) = file {
                        if let Some(index) = ctx
                            .previously_loaded_files
                            .iter()
                            .position(|e| *e == file.path())
                        {
                            ctx.previously_loaded_files.remove(index);
                        }
                        ctx.previously_loaded_files.push(PathBuf::from(file.path()));
                        #[cfg(feature = "log")]
                        ctx.logger.info(format!(
                            "Loading ROM {}",
                            file.path()
                                .to_str()
                                .expect("Unable to convert path into str")
                        ));
                        let core_type = if let Some(core_type) = core_type {
                            core_type
                        } else {
                            todo!()
                        };
                        Some((file, core_type))
                    } else {
                        None
                    }
                };
            if ui.button("Load ROM") {
                let opened = open_file(ctx, None);
                if let Some((_file, _kind)) = opened {
                    todo!()
                } else {
                    #[cfg(feature = "log")]
                    ctx.logger.warning("No ROM file opened.")
                }
            }
            ui.menu("Load ROM As", || {
                #[cfg(feature = "nds-core")]
                if ui.button(NDS_FILTER.name) {
                    if let Some((file, _)) = open_file(ctx, Some(CoreType::Nds)) {
                        let read = pollster::block_on(file.read());
                        *core = Core::Core(Box::new(NDSFrontend::new(read, &ctx.logger)));
                    } else {
                        ctx.logger.warning("No ROM file opened for NDS core.");
                    }
                }
            });
            ui.menu("Previously opened", || {
                if ctx.previously_loaded_files.is_empty() {
                    ui.enabled(false, || ui.text("(empty)"));
                } else {
                    for path in &ctx.previously_loaded_files {
                        if ui.button(
                            path.as_os_str()
                                .to_str()
                                .expect("Unable to convert into str"),
                        ) {
                            if let Ok(read) = std::fs::read(path) {
                                todo!()
                            } else {
                                #[cfg(feature = "log")]
                                ctx.logger.warning("Invalid path to ROM");
                            }
                        }
                    }
                }
            });
            ui.enabled(core.is_core(), || {
                if ui.button("Close core") {
                    *core = Core::None;
                }
            });
        });
        // Emulation subwindow.
        match core {
            Core::None => ui.enabled(false, || {
                ui.button("Emulation");
                #[cfg(debug_assertions)]
                ui.button("Debug");
            }),
            Core::Core(_) => {
                ui.menu("Emulation", || {});
                #[cfg(debug_assertions)]
                ui.menu("Debug", || {});
            }
        }
        // Config subwindow.
        if ui.button("Config") {
            ctx.config_window_active = !ctx.config_window_active;
        };
        // Help subwindow.
        if ui.button("Help") {
            ctx.help_window_active = !ctx.help_window_active;
        }
    });
}

pub(crate) fn ui(imgui_ctx: &mut ImguiCtx, ctx: &mut Ctx, core: &mut Box<dyn CoreFrontend>) {
    // Draw debug panels.
    #[cfg(debug_assertions)]
    core.update_panels(imgui_ctx);

    let ui = imgui_ctx.ui();

    if cfg!(debug_assertions) && !ctx.fullscreen {
        // subwindow display.
        imgui::Window::new("Display").build(ui, || {
            ui.text("test!");
        });
    } else {
        // fullscreen display.
        todo!()
    }
}
