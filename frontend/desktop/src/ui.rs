pub mod gfx;

mod window_loop;

use crate::common::CoreFrontend;
use crate::cores;
use crate::{
    cores::{Core, CoreType},
    Ctx,
};
#[cfg(feature = "nds-core")]
use imgui::StyleColor;
use imgui::Ui;
use rfd::FileHandle;
use std::path::{Path, PathBuf};
use util::Logger;

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
        ctx.logger.warning("Unable to load file from dialogue");
        None
    }
}

fn main_menu(
    ui: &Ui,
    ctx: &mut Ctx,
    core: &mut Core,
    spawn_core: &mut Option<(Vec<u8>, CoreType)>,
) {
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
                ui.menu("Debug", || core.debug_views_menu(ui));
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

pub fn main() {
    let mut core = Core::None;
    let mut ctx = Ctx {
        previously_loaded_files: vec![],
        config_window_active: false,
        help_window_active: false,
        #[cfg(feature = "log")]
        logger: Logger::default(),
        fullscreen: false,
    };
    let window_loop = window_loop::Builder {
        gfx_builder: gfx::Builder {},
        clear_colour: wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
    };
    window_loop.run(move |win, ui, control_flow| {
        let mut spawn_core = None;
        main_menu(ui, &mut ctx, &mut core, &mut spawn_core);
        {
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

        if let Some((rom, ct)) = spawn_core {
            fn b<T>(val: T) -> Box<T> {
                Box::new(val)
            }
            let logger = &ctx.logger;
            core = Core::Core(match ct {
                CoreType::Nds => b(cores::nds::new(&mut win.gfx, rom, logger)),
            });
        }

        match &mut core {
            Core::Core(core) => {
                core.sync_core();
                // Draw debug panels.
                #[cfg(feature = "debug-views")]
                core.debug_views(ui);
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
            Core::None => {}
        }
    });
}
