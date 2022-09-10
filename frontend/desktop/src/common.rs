pub mod debug_view;
pub mod demsgq;

use imgui::Ui;

pub trait CoreFrontend {
    fn debug_views(&mut self, gui: &Ui);

    fn debug_views_menu(&mut self, gui: &Ui);

    fn sync_core(&mut self);
}
