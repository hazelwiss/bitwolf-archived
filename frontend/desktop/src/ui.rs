use crate::{
    cores::{Core, CoreType},
    Ctx,
};
use common::CoreFrontend;
#[cfg(feature = "nds-core")]
use gui::{imgui::StyleColor, window_loop::ImguiCtx};
use rfd::FileHandle;
use std::path::{Path, PathBuf};

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

fn push_to_recently_opened(ctx: &mut Ctx, opened: (&Path, CoreType)) {
    let (file, ct) = opened;
    if let Some(index) = ctx.previously_loaded_files.iter().position(|e| e.0 == file) {
        ctx.previously_loaded_files.remove(index);
    }
    ctx.previously_loaded_files.push((PathBuf::from(file), ct));
}

fn open(ctx: &mut Ctx, core_type: Option<CoreType>) -> Option<(Vec<u8>, CoreType)> {
    let default_fd = rfd::AsyncFileDialog::new().set_title("Load ROM");
    let fd = if let Some(core_type) = core_type {
        match core_type {
            #[cfg(feature = "nds-core")]
            CoreType::Nds => default_fd.add_filter(NDS_FILTER.name, NDS_FILTER.extensions),
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
        push_to_recently_opened(ctx, (file.path(), core_type));
        Some((pollster::block_on(file.read()), core_type))
    } else {
        #[cfg(feature = "log")]
        ctx.logger
            .warning(format!("Unable to load file from dialogue"));
        None
    }
}

pub fn main_menu(
    gui: &ImguiCtx,
    ctx: &mut Ctx,
    core: &mut Core,
    spawn_core: &mut Option<(Vec<u8>, CoreType)>,
) {
    let ui = gui.ui();
    ui.main_menu_bar(|| {
        let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
        // File submenu.
        ui.menu("File", || {
            let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
            let [pos_x, pos_y] = ui.cursor_pos();
            let width = ui.calc_item_width();
            ui.set_cursor_pos([pos_x - width * 0.015, pos_y]);
            if ui.button("Load ROM") {
                if let Some((rom, ct)) = open(ctx, None) {
                    *spawn_core = Some((rom, ct));
                }
            }
            ui.menu("Load ROM As", || {
                #[cfg(feature = "nds-core")]
                if ui.button(NDS_FILTER.name) {
                    if let Some((rom, ct)) = open(ctx, Some(CoreType::Nds)) {
                        *spawn_core = Some((rom, ct));
                    }
                }
            });
            ui.enabled(!ctx.previously_loaded_files.is_empty(), || {
                ui.menu("Previously opened", || {
                    if ctx.previously_loaded_files.is_empty() {
                        ui.enabled(false, || ui.text("(empty)"));
                    } else {
                        for prev in ctx.previously_loaded_files.clone().into_iter().rev() {
                            let (path, kind) = prev;
                            if ui.button(
                                path.as_os_str()
                                    .to_str()
                                    .expect("Unable to convert into str"),
                            ) {
                                if path.exists() {
                                    if let Ok(read) = std::fs::read(path.clone()) {
                                        *spawn_core = Some((read, kind));
                                        push_to_recently_opened(ctx, (&path, kind));
                                        break;
                                    } else {
                                        #[cfg(feature = "log")]
                                        ctx.logger.warning(format!("Unable to open file {path:?}"))
                                    }
                                } else {
                                    #[cfg(feature = "log")]
                                    ctx.logger.warning("Invalid path to ROM");
                                }
                            }
                        }
                    }
                });
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
                #[cfg(feature = "debug-views")]
                ui.button("Debug");
            }),
            Core::Core(core) => {
                ui.menu("Emulation", || {});
                #[cfg(feature = "debug-views")]
                ui.menu("Debug", || core.debug_views_menu(gui));
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
    #[cfg(feature = "debug-views")]
    core.debug_views(imgui_ctx);

    let ui = imgui_ctx.ui();

    if cfg!(debug_assertions) && !ctx.fullscreen {
        // subwindow display.
        gui::imgui::Window::new("Display").build(ui, || {
            ui.text("test!");
        });
    } else {
        // fullscreen display.
        todo!()
    }
}
