pub mod gfx;

use crate::{
    cli::CliArgs,
    config,
    debug_views::{DebugViews, DebugViewsBuilder},
    emu::{self, SharedState},
};
use ::imgui::Ui;
use anyhow::anyhow;
use crossbeam_channel::{Receiver, TryRecvError};
use gfx::Window;
use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Instant,
};
#[allow(unused)]
use util::log::{self, info};
use util::log::{error, Logger};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct EmuState {
    shared_state: SharedState,
    jhandle: JoinHandle<()>,
    receiver: Receiver<emu::Message>,
}

impl EmuState {
    fn new(config: &config::GlobalConfig, log: Logger, rom: PathBuf) -> anyhow::Result<Self> {
        let err = anyhow!("failed to load rom with path {rom:?}");
        let core = bitwolf_core::CoreBuilder::new()
            .rom(if let Ok(rom) = fs::read(rom) {
                rom
            } else {
                return Err(err);
            })
            .build();

        let (sender, receiver) = crossbeam_channel::bounded(25);

        let shared_state = SharedState {
            running: Arc::new(AtomicBool::new(true)),
        };
        let emu_log = log.clone();
        let emu_shared_state = shared_state.clone();
        let jhandle = thread::Builder::new()
            .name("bitwolf-core".to_string())
            .spawn(move || {
                emu::run(core, emu_log, emu_shared_state, sender);
            })
            .expect("failed to start core thread.");
        Ok(Self {
            shared_state,
            jhandle,
            receiver,
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
    window: Window,
    last_frame: Instant,

    log: Logger,
    debug_views: DebugViews,

    emu_state: Option<EmuState>,

    config: config::GlobalConfig,
}

pub fn run(#[allow(unused)] log: log::Logger, cli_args: CliArgs) {
    let config = if let Some(path) = cli_args.config {
        config::load_config(&path)
    } else {
        config::global_config()
    };

    let debug_views = DebugViewsBuilder::default().build();

    let emu_state = if let Some(rom) = cli_args.rom {
        match EmuState::new(&config, log.clone(), rom) {
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

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Bitwolf")
        .build(&event_loop)
        .expect("failure to build winit window");
    let window = Window::new(window, wgpu::Color::BLACK);
    let last_frame = Instant::now();

    let mut gui = GuiState {
        window,
        last_frame,
        log,
        debug_views,
        emu_state,
        config,
    };

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match &event {
            Event::WindowEvent { event, .. } => match event {
                #[allow(unused)]
                WindowEvent::MouseInput { state, button, .. } => {}
                #[allow(unused)]
                WindowEvent::KeyboardInput { input, .. } => {}
                _ => {}
            },
            Event::MainEventsCleared => gui.window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                gui.window
                    .imgui
                    .io_mut()
                    .update_delta_time(now - gui.last_frame);
                gui.last_frame = now;
                gui.window.draw(|ui, gfx| {
                    ui_update(&gui.log, gfx, &mut gui.emu_state, &mut gui.debug_views, ui);
                });
            }
            Event::LoopDestroyed => *flow = ControlFlow::Exit,
            _ => {}
        }
        gui.window.handle_event(event, flow);
    });
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn ui_update(
    log: &Logger,
    gfx: &mut gfx::GfxContext,
    emu_state: &mut Option<EmuState>,
    views: &mut DebugViews,
    ui: &Ui,
) {
    ui.main_menu_bar(|| {
        ui.menu("file", || {});
        ui.menu("options", || {});
        if emu_state.is_some() {
            views.menu(ui);
        }
    });

    if let Some(state) = emu_state {
        let EmuState {
            shared_state,
            jhandle,
            receiver,
        } = state;

        views.draw(gfx, ui);

        match receiver.try_recv() {
            Ok(msg) => match msg {
                emu::Message::DebugView(msg) => views.update_state(msg),
            },
            Err(TryRecvError::Empty) => {}
            Err(e) => error!(
                log,
                "failing to receive message from backend with error: {e:?}"
            ),
        }
    }
}
