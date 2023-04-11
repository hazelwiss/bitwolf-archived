pub mod debug;

mod menu;
mod window;

use crate::config::Config;
use crate::frontend::Frontend;
use crate::{cla, config};
use std::path::Path;
use winit::event_loop::EventLoop;

struct State {
    config: config::Config,
    frontend: Frontend,
}

pub fn main() {
    // Parse command line arguments and access emulator config.
    let cla = cla::from_env();
    let config = config::from_env().with_cla(cla);
    // Gui state.
    let state = State {
        frontend: Frontend::nds(
            std::fs::read(&config.load_rom.as_ref().expect("expected initial rom cli command").rom)
                .expect("failed to read rom")
                .into_boxed_slice(),
        ),
        config,
    };
    // Initialize winit event loop, window and imgui context.
    let event_loop = EventLoop::new();
    let mut imgui_ctx = imgui::Context::create();
    let window = window::Builder::new()
        .build(&event_loop, &mut imgui_ctx)
        .expect("failed to create window");
    window::run(
        event_loop,
        window,
        state,
        imgui_ctx,
        // Frame function.
        |state, ui, io, _control_flow| {
            menu::main_bar(state, ui);
            state.frontend.update(ui, io);
        },
        // Input function.
        |state| info!("input!"),
        // Exit function.
        |state| info!("exit!"),
    );
}
