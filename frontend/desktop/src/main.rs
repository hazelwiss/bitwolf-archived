#![feature(if_let_guard)]
mod default_backend;
mod menu;

fn main() {
    // Backend.
    let mut frontend: common_frontend::FrontendBox =
        Box::new(default_backend::EmptyFrontend::new());

    // Create imgui rendering window.
    let ctx = imgui::Context::spawn_with_window();

    // Start imgui rendering window event loop.
    ctx.run(
        // Ran on each draw.
        move |draw_ctx| {
            // Draws main menu bar.
            menu::menu(draw_ctx, &mut frontend);
        },
        // Ran whenever input was received.
        move |_input| {},
    );
}
