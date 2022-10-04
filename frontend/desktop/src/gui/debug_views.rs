mod disassembly;

use super::gfx::window::WindowGfx;
use core::ops::{Deref, DerefMut};
use imgui::Ui;

trait DebugView: Default {
    fn draw(&mut self, window: &mut WindowGfx, ui: &Ui);
}

#[derive(Default)]
struct DebugViewHandle<T: DebugView> {
    view: T,
    open: bool,
}

impl<T: DebugView> Deref for DebugViewHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<T: DebugView> DerefMut for DebugViewHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

macro_rules! debug_views {
    ($($ident:ident, $print:literal, $ty:ty);* $(;)?) => {
        #[derive(Default)]
        pub struct DebugViews {
            $(
                $ident: DebugViewHandle<$ty>,
            )*
        }

        impl DebugViews{
            pub fn draw(&mut self, window: &mut WindowGfx, ui: &Ui){
                $(
                    {
                        let cur = &mut self.$ident;
                        let open = &mut cur.open;
                        let view = &mut cur.view;
                        if *open{
                            imgui::Window::new($print).opened(open) .build(ui, ||{
                                view.draw(window, ui);
                            });
                        }
                    }
                )*
            }

            pub fn menu(&mut self, ui: &Ui){
                ui.menu("debug-views", ||{
                    $(
                        {
                            let cur = &mut self.$ident;
                            let name = $print;
                            ui.checkbox(name, &mut cur.open);
                        }
                    )*
                });
            }
        }
    };
}

debug_views! {
    disassembly, "disassmebly-view", disassembly::DVDisasm;
}
