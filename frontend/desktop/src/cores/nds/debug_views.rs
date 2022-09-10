pub mod arm9_disasm;
pub mod arm9_state;
pub mod metadata;

use crate::{common::debug_view::View, ui::gfx::GfxContext};
use imgui::Ui;

macro_rules! create_views {
    ($($name:ident, $ty:ty, $msg:ident $(;)?)*) => {
        #[derive(Default)]
        struct UIElement<T: View>{
            elem: T,
            state: T::MutableState,
            opened: bool,
        }

        impl<T: View> UIElement<T>{
            fn new(elem: T) -> Self{
                Self{
                    elem,
                    state: <T as View>::MutableState::default(),
                    opened: false,
                }
            }
        }

        pub enum DebugViewMsg{
            $(
                $msg (<$ty as View>::MutableState)
            ),*
        }

        pub struct Builder{
            $(
                pub $name: $ty
            ),*
        }

        impl Builder{
            pub fn build(self) -> UIState{
                UIState {
                    $(
                        $name: UIElement::new(self.$name)
                    ),*
                }
            }
        }

        pub struct UIState {
            $(
                $name: UIElement<$ty>,
            )*
        }

        impl UIState{
            pub fn views(&mut self, ui: &Ui){
                $(
                    let UIElement{
                        elem,
                        state,
                        opened
                    } = &mut self.$name;
                    let window = <$ty>::construct_window(imgui::Window::new(<$ty>::window_title()).
                        title_bar(true).menu_bar(true).collapsible(true));
                    if *opened{
                        window.opened(opened) .build(ui, ||{
                            ui.menu_bar(||{
                                elem.menu_bar(ui, &state);
                            });
                            elem.view(ui, &state);
                        });
                    }
                )*
            }

            pub fn debug_view_submenu(&mut self, ui: &Ui){
                $(
                    let dv = &mut self.$name;
                    let opened = &mut dv.opened;
                    let id = <$ty>::window_title();
                    ui.checkbox(id, opened);
                )*
            }

            pub fn recv_message(&mut self, msg: DebugViewMsg){
                use DebugViewMsg::*;
                match msg{
                    $(
                        $msg(inner) => {
                            let UIElement{
                                elem,
                                state,
                                ..
                            } = &mut self.$name;
                            let mut old = inner;
                            core::mem::swap(&mut old, state);
                            elem.on_state_changed(old, &state);
                        }
                    )*
                };
            }
        }
    };
}

create_views!(
    arm9_disasm, arm9_disasm::Arm9Disasm, Arm9Disasm;
    arm9_state, arm9_state::Arm9State, Arm9State;
    metadata, metadata::Metadata, Metadata;
);
