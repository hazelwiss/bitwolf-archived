use imgui::DrawContext;

pub trait Emulation {
    fn emulatable(&self) -> bool {
        true
    }

    fn menu_emulation(&mut self, draw_ctx: &mut DrawContext);
}
