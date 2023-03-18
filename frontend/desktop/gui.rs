mod menu;
mod window;

use crate::config::Config;
use crate::core::{self, Core___};
use crate::state::ProgramState;
use crate::{cla, config};
use std::path::Path;
use winit::event_loop::EventLoop;

pub fn main() {
    // Parse command line arguments and access emulator config.
    let cla = cla::from_env();
    let config = config::from_env();
    // Create global program state.
    let state = ProgramState {
        config,
        core: core::nds::NDS::new(),
    };
    run_core_loop(state);
}

fn run_core_loop<C: Core___ + 'static>(state: ProgramState<C>) {
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
            Core___::draw_debug(state, ui, io);
            Core___::run_until_sync(state);
        },
        // Input function.
        |state| {},
        // Exit function.
        |state| {},
    );
}
