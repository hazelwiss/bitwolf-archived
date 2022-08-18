use imgui::StyleColor;

use crate::cores;
use crate::{common::CoreFrontend, window_loop::ImguiCtx, Core, Ctx};
#[cfg(feature = "nds-core")]
use cores::nds::NDS;

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
        // File submenu.
        ui.menu("File", || {
            let default_fd = rfd::AsyncFileDialog::new();
            let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
            let [pos_x, pos_y] = ui.cursor_pos();
            let width = ui.calc_item_width();
            ui.set_cursor_pos([pos_x - width * 0.015, pos_y]);
            if ui.button("Load ROM") {
                let mut fd = default_fd.clone().set_title("Load ROM");
                for filter in ALL_FILTERS {
                    fd = fd.add_filter(filter.name, filter.extensions);
                }
                let file = pollster::block_on(fd.pick_file());
                if let Some(file) = file {
                    #[cfg(feature = "logging")]
                    ctx.logger.info(format!(
                        "Loading ROM {}",
                        file.path()
                            .to_str()
                            .expect("Unable to convert path into str")
                    ));
                    todo!()
                } else {
                    #[cfg(feature = "logging")]
                    ctx.logger.warning("No ROM file opened.")
                }
            }
            ui.menu("Load ROM As", || {
                #[cfg(feature = "nds-core")]
                if ui.button(NDS_FILTER.name) {
                    let file = pollster::block_on(
                        default_fd
                            .set_title("Load ROM (NDS)")
                            .add_filter(NDS_FILTER.name, NDS_FILTER.extensions)
                            .pick_file(),
                    );
                    if let Some(file) = file {
                        #[cfg(feature = "logging")]
                        ctx.logger.info(format!(
                            "Loading ROM {} (NDS core)",
                            file.path()
                                .to_str()
                                .expect("Unable to convert path into str")
                        ));
                        let read = pollster::block_on(file.read());
                        *core = Core::Core(Box::new(NDS::new(read, &ctx.logger)));
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
                        if ui.button(format!(
                            "{}",
                            path.as_os_str()
                                .to_str()
                                .expect("Unable to convert into str")
                        )) {
                            todo!();
                        }
                    }
                }
            });
        });
        let _style_col = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 0.0]);
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
    #[cfg(feature = "debug")]
    core.update_panels(imgui_ctx);

    let ui = imgui_ctx.ui();

    if cfg!(feature = "debug") {
        // either fullscreen or subwindow display.
        imgui::Window::new("Display").build(ui, || {
            ui.text("test!");
        });
    } else {
        // fullscreen display.
        todo!()
    }
}
