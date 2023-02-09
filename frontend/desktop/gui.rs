mod debug;
mod menu;
mod window;

use std::path::Path;

use crate::config::Config;
use crate::core::{Backend, CoreBuilder, Runner};
use crate::{cla, config};
use winit::event_loop::EventLoop;

enum CoreState {
    Runner(Runner),
    None,
}

impl CoreState {
    fn empty(&self) -> bool {
        match self {
            CoreState::Runner(_) => true,
            CoreState::None => false,
        }
    }
}

struct GUIState {
    core: CoreState,
    config: Config,
    debug: debug::DebugUI,
}

impl GUIState {
    fn spawn_core_with_config(&mut self, backend: Backend, rom: &Path) {
        self.core = CoreState::Runner(
            CoreBuilder::new(backend, rom.to_path_buf())
                .from_config(&self.config)
                .build_threaded(),
        )
    }
}

fn spawn_imgui_ctx() -> imgui::Context {
    let imgui_ctx = imgui::Context::create();
    imgui_ctx
}

pub fn main() {
    // Initialize winit event loop, window and imgui context.
    let event_loop = EventLoop::new();
    let mut imgui_ctx = spawn_imgui_ctx();
    let window = window::Builder::new()
        .build(&event_loop, &mut imgui_ctx)
        .expect("failed to create window");
    // Parse command line arguments and access emulator config.
    let cla = cla::from_env();
    let config = config::from_env();
    // Create global GUI state.
    let gui_state = GUIState {
        core: if let Some(builder) = CoreBuilder::from_cla(&cla) {
            CoreState::Runner(builder.from_config(&config).build_threaded())
        } else {
            CoreState::None
        },
        config,
        debug: debug::DebugUI::new(),
    };
    window::run(
        event_loop,
        window,
        gui_state,
        imgui_ctx,
        // Frame function.
        |state, ui, io, _control_flow| {
            menu::main_bar(state, ui);
            debug::draw(state, ui);
        },
        // Input function.
        |state| {},
        // Exit function.
        |state| {},
    );
}
