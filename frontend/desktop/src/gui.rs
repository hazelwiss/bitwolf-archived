pub mod window;

use self::window::WindowBuilder;
use crate::{
    cli::CliArgs,
    config,
    debug_views::{self, DebugViews, DebugViewsBuilder},
    emu::{self, SharedState},
};
use ::imgui::Ui;
use anyhow::anyhow;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};
#[allow(unused)]
use util::log::{self, info};
use util::log::{error, Logger};

struct EmuState {
    shared_state: SharedState,
    jhandle: JoinHandle<()>,
    receiver: Receiver<emu::EmuMsg>,
    sender: Sender<emu::FrontendMsg>,
}

impl EmuState {
    fn new(
        config: &config::GlobalConfig,
        debug_views: &mut DebugViews,
        log: Logger,
        rom: PathBuf,
    ) -> anyhow::Result<Self> {
        let err = anyhow!("failed to load rom with path {rom:?}");
        let core = bitwolf_core::CoreBuilder::new()
            .rom(if let Ok(rom) = fs::read(rom) {
                rom
            } else {
                return Err(err);
            })
            .build();

        *debug_views = DebugViewsBuilder {
            cartridge_view: debug_views::cartridge::State {
                cartridge_header: bitwolf_core::debug::cartridge_info::cartridge_header(&core),
            },
        }
        .build();

        let (emu_sender, fe_receiver) = crossbeam_channel::bounded(25);
        let (fe_sender, emu_receiver) = crossbeam_channel::bounded(25);

        let shared_state = SharedState {
            running: Arc::new(AtomicBool::new(true)),
        };

        let emu_shared_state = shared_state.clone();
        let jhandle = thread::Builder::new()
            .name("bitwolf-core".to_string())
            .spawn(move || emu::run(core, log, emu_shared_state, emu_sender, emu_receiver))
            .expect("failed to start core thread.");
        Ok(Self {
            shared_state,
            jhandle,
            receiver: fe_receiver,
            sender: fe_sender,
        })
    }

    fn stop(&self) {
        self.shared_state.running.store(false, Ordering::Relaxed);
    }

    fn invalidate(self) {
        self.stop();
        self.jhandle
            .join()
            .expect("unable to join emulator thread.");
    }
}

struct GuiState {
    log: Logger,
    views: DebugViews,

    emu_state: Option<EmuState>,

    config: config::GlobalConfig,
}

pub fn run(#[allow(unused)] log: log::Logger, cli_args: CliArgs) {
    let config = if let Some(path) = cli_args.config {
        config::load_config(&path)
    } else {
        config::global_config()
    };

    let mut debug_views = DebugViewsBuilder::default().build();

    let emu_state = if let Some(rom) = cli_args.rom {
        match EmuState::new(&config, &mut debug_views, log.clone(), rom) {
            Ok(state) => Some(state),
            Err(e) => {
                error!(
                    log,
                    "failed to create emulator state with command line argument. Error:\n{e}"
                );
                None
            }
        }
    } else {
        None
    };

    let mut gui = GuiState {
        log,
        views: debug_views,
        emu_state,
        config,
    };

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Bitwolf")
        .build(&event_loop)
        .expect("failure to build winit window");
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(Some(PathBuf::from("imgui.ini")));
    let hpdi_factor = window.scale_factor();
    imgui.io_mut().font_global_scale = (1.0 / hpdi_factor) as f32;
    let font_size = (13.0 * hpdi_factor) as f32;
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

    WindowBuilder {
        window: window::Window::new(&mut imgui, window, wgpu::Color::BLACK),
        event_loop: event_loop,
        imgui,
    }
    .run(
        gui,
        |_state, event, _window, _imgui| println!("event! {event:?}"),
        move |state, ui, window| {
            let _ = 0;
            ui_update(window, state, ui)
        },
    );
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn ui_update(window: &mut window::Window, state: &mut GuiState, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu("file", || {});
        ui.menu("options", || {});
        if state.emu_state.is_some() {
            state.views.menu(ui);
        }
    });

    if let Some(emu) = &state.emu_state {
        let EmuState {
            shared_state,
            jhandle,
            receiver,
            sender,
        } = emu;

        state.views.draw(window, ui);

        state.views.config(&state.log, &emu.sender);
        match receiver.try_recv() {
            Ok(msg) => match msg {
                emu::EmuMsg::DebugView(msg) => state.views.update_state(msg),
            },
            Err(TryRecvError::Empty) => {}
            Err(e) => panic!("failing to receive message from backend with error: {e:?}"),
        }
    }
}
