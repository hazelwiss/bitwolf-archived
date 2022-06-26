#![feature(if_let_guard)]

mod backend_types;
mod default_backend;
mod frontend_creator;
mod menu;
mod msg_receiver;
mod proc_flags;

fn main() {
    // Spawn the environment based on input flags to the binary.
    let proc_flags::Environment {
        frontend_box: mut frontend,
        imgui_ctx: ctx,
    } = proc_flags::env_from_flags();

    // Asynchronous file reader.
    let mut file_reader = file_reader::FileReader::<backend_types::Types>::new(5);

    // Start imgui rendering window event loop.
    ctx.run(
        // Ran on each draw.
        move |draw_ctx| {
            // Receive files from message queue.
            msg_receiver::files::receive(&mut file_reader, &mut frontend, draw_ctx.resources());
            // Draws main menu bar.
            menu::menu(draw_ctx, &mut file_reader, &mut frontend);
            // Draws the frontend.
            frontend.draw(draw_ctx);
            // Update backend.
            frontend.update();
        },
        // Ran whenever input was received.
        move |_input| {},
    );
}
