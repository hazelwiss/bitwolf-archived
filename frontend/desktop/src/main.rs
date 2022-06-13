#![feature(if_let_guard)]
mod default_backend;
mod menu;

fn main() {
    // Backend.
    let mut backend: common_frontend::FrontendBox = Box::new(default_backend::EmptyFrontend::new());

    // Create imgui rendering window.
    let ctx = imgui::Context::spawn_with_window();

    // Start imgui rendering window event loop.
    ctx.run(
        // Ran on each draw.
        move |ctx| {
            menu::draw(ctx, &mut backend)
            // rustfmt is being naughty removing my brackets if I don't keep this line!
        },
        // Ran whenever input was received.
        move |_input| {},
    );
}
