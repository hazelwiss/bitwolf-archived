#![feature(if_let_guard)]

mod backend_types;
mod default_backend;
mod display;
mod file_reader;
mod frontend_creator;
mod menu;
mod msg_receiver;
mod proc_flags;

fn main() {
    // Spawn the environment based on input flags to the binary.
    let proc_flags::Environment {
        frontend_box: frontend,
        imgui_ctx: ctx,
    } = proc_flags::env_from_flags();

    // Asynchronous file reader.
    let mut file_reader = file_reader::FileReader::<backend_types::Types>::new(5);

    // Start imgui rendering window event loop.
    ctx.run(
        frontend,
        // Ran on each draw.
        move |frontend, draw_ctx| {
            // Receive files from message queue.
            msg_receiver::files::receive(&mut file_reader, frontend, draw_ctx.resources());
            // Draws main menu bar.
            menu::menu(draw_ctx, &mut file_reader, frontend);
            // Draws the frontend.
            if frontend.is_fullscreen() {
                // Renders the fb in full picture mode.
                display::full(frontend, draw_ctx);
            } else {
                // Renders the fb in a window.
                display::window(frontend, draw_ctx);
            }
            // Display debug panels/windows.
            if frontend.is_debugging() {
                frontend.get_inner_mut().draw_debug(draw_ctx);
            }
            // Update backend.
            frontend.get_inner_mut().update();
        },
        // Ran whenever input was received.
        move |frontend, input| frontend.get_inner_mut().input(input),
    );
}
