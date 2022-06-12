use super::{Backend, DrawContext};

pub fn draw(draw_ctx: &DrawContext, _: &mut Backend) {
    draw_ctx.ui().menu("Help", || {});
}
