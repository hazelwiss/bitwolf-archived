use crate::ui::gfx::GfxContext;
use imgui::Ui;

pub trait View {
    type MutableState: Default;

    fn destroy(gfx: &mut GfxContext);

    fn window_title() -> &'static str;

    fn construct_window<T: AsRef<str>>(window: imgui::Window<'_, T>) -> imgui::Window<'_, T>;

    fn on_state_changed(&mut self, old_state: Self::MutableState, new_state: &Self::MutableState);

    fn menu_bar(&mut self, ui: &Ui, state: &Self::MutableState);

    fn view(&mut self, ui: &Ui, state: &Self::MutableState);
}
