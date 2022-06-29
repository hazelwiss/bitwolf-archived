use imgui::DrawContext;

pub trait Debuggable {
    fn debuggable(&self) -> bool {
        true
    }

    fn menu_debug(&mut self, draw_ctx: &mut DrawContext);
}
