mod open;

use super::DrawContext;
use crate::backends::{Backend, BackendType};

pub fn draw(draw_ctx: &mut DrawContext, backend: &mut Backend) {
    draw_ctx.ui().menu("File", || {
        draw_ctx.ui().menu("Open", || {
            draw_ctx.ui().menu("Nintendo", || {
                if draw_ctx.ui().button("gbc") {
                    backend.swap(
                        draw_ctx,
                        BackendType::NintendoGBC(open::nintendo::gbc::open(
                            &std::path::PathBuf::from(""),
                        )),
                    );
                }
            });
        })
    });
}
