#![feature(if_let_guard)]

mod backend_types;
mod default_backend;
mod menu;
mod msg_receiver;

use common_frontend::FrontendBox;

fn main() {
    // Backend.
    let mut frontend: FrontendBox = FrontendBox::new(default_backend::EmptyFrontend::new());

    // Create imgui rendering window.
    let ctx = imgui::Context::spawn_with_window();

    // Asynchronous file reader.
    let mut file_reader = file_reader::FileReader::<backend_types::Types>::new(100);

    // Start imgui rendering window event loop.
    ctx.run(
        // Ran on each draw.
        move |draw_ctx| {
            // Draws main menu bar.
            menu::menu(draw_ctx, &mut file_reader, &mut frontend);
            msg_receiver::files::receive(&mut file_reader, &mut frontend);
        },
        // Ran whenever input was received.
        move |_input| {},
    );
}
