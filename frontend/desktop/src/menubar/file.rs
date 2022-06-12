use super::{Backend, DrawContext};

pub fn draw(draw_ctx: &mut DrawContext, _: &mut Backend) {
    draw_ctx.ui().menu("File", || {});
}
