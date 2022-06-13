mod backends;
mod config;
mod file_explorer;
mod menubar;

fn main() {
    // Backend.
    let mut backend = backends::Backend::none();

    // Create imgui rendering window.
    let ctx = imgui::Context::spawn_with_window();

    // Start imgui rendering window event loop.
    ctx.run(
        // Ran on each draw.
        move |ctx| {
            menubar::draw(ctx, &mut backend);
            backend.update();
        },
        // Ran whenever input was received.
        move |_input| {},
    );
}
