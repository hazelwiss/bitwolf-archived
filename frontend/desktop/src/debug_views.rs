#![allow(dead_code)]

pub mod cartridge;
pub mod disassembly;
pub mod registers;

use crate::{emu::FrontendMsg, gui::window::Window};
use core::ops::{Deref, DerefMut};
use crossbeam_channel::{Sender, TrySendError};
use imgui::Ui;
use util::log::{warn, Logger};

pub trait DebugView: Default {
    type State: Default;
    type Conf: Default;

    fn draw(
        &mut self,
        state: &mut Self::State,
        global_state: &GlobalState,
        window: &mut Window,
        ui: &Ui,
    );

    fn on_change(&mut self, old: Self::State, new: &mut Self::State);

    fn config(&self, state: &Self::State) -> Option<Self::Conf>;
}

pub trait GlobalStateData: Default {
    type State: Default;

    fn on_change(&mut self, old: Self::State);

    fn set_state(&mut self, new: Self::State);

    fn get_state(&self) -> Self::State;
}

#[derive(Default)]
struct DebugViewHandle<T: DebugView> {
    view: T,
    state: T::State,
    open: bool,
}

impl<T: DebugView> DebugViewHandle<T> {
    fn from_state(state: T::State) -> Self {
        Self {
            state,
            view: Default::default(),
            open: false,
        }
    }
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
    (draw $global_state:expr, $window:expr, $ui:expr, $val:expr, $print:literal) => {
        {
            let cur = &mut $val;
            let open = &mut cur.open;
            let view = &mut cur.view;
            let state = &mut cur.state;
            if *open{
                imgui::Window::new($print).opened(open).build($ui, ||{
                    view.draw(state, $global_state, $window, $ui);
                });
            }
        }
    };
    (
        $(data $da_ident:ident, $da_msg:ident, $da_ty:ty);*;
        $(update $up_ident:ident, $up_msg:ident, $up_print:literal, $up_ty:ty);*;
        $(instance $in_ident:ident, $in_print:literal, $in_ty:ty);* $(;)?
    ) => {
        #[derive(Debug)]
        pub enum DebugViewMsg{
            $(
                $up_msg (<$up_ty as DebugView>::State),
            )*
            $(
                $da_msg (<$da_ty as GlobalStateData>::State),
            )*
        }

        #[derive(Debug)]
        pub enum DebugViewConfMsg{
            $(
                $up_msg (<$up_ty as DebugView>::Conf),
            )*
        }

        #[derive(Default)]
        pub struct GlobalState{
            $(
                $da_ident: $da_ty
            ),*
        }

        #[derive(Default)]
        pub struct DebugViewsConfState{
            $(
                pub $up_ident: <$up_ty as DebugView>::Conf
            ),*
        }

        impl DebugViewsConfState{
            pub fn update(&mut self, msg: DebugViewConfMsg){
                use DebugViewConfMsg::*;
                match msg{
                    $(
                        $up_msg (conf) => self.$up_ident = conf,
                    )*
                }
            }
        }

        #[derive(Default)]
        pub struct DebugViewsBuilder{
            $(
                pub $in_ident: <$in_ty as DebugView>::State,
            ),*
        }

        impl DebugViewsBuilder{
            pub fn build(self) -> DebugViews{
                DebugViews{
                    $(
                        $in_ident: DebugViewHandle::from_state(self.$in_ident),
                    )*
                    $(
                        $up_ident: Default::default(),
                    )*
                    global_state: GlobalState::default(),
                }
            }
        }

        pub struct DebugViews {
            $(
                $up_ident: DebugViewHandle<$up_ty>,
            )*
            $(
                $in_ident: DebugViewHandle<$in_ty>,
            )*
            global_state: GlobalState,
        }

        impl DebugViews{
            pub fn draw(&mut self, window: &mut Window, ui: &Ui){
                $(
                    debug_views!(draw &self.global_state, window, ui, self.$up_ident, $up_print);
                )*
                $(
                    debug_views!(draw &self.global_state, window, ui, self.$in_ident, $in_print);
                )*
            }

            pub fn menu(&mut self, ui: &Ui){
                ui.menu("debug-views", ||{
                    $(
                        {
                            let cur = &mut self.$up_ident;
                            let name = $up_print;
                            ui.checkbox(name, &mut cur.open);
                        }
                    )*
                    $(
                        {
                            let cur = &mut self.$in_ident;
                            let name = $in_print;
                            ui.checkbox(name, &mut cur.open);
                        }
                    )*
                });
            }

            pub fn update_state(&mut self, state: DebugViewMsg){
                use DebugViewMsg::*;
                match state{
                    $(
                        $up_msg (state) => {
                            let mut old = state;
                            std::mem::swap(&mut self.$up_ident.state, &mut old);
                            self.$up_ident.view.on_change(old, &mut self.$up_ident.state);
                        },
                    )*
                    $(
                        $da_msg (state) => {
                            let cur = &mut self.global_state.$da_ident;
                            let old = GlobalStateData::get_state(cur);
                            GlobalStateData::set_state(cur, state);
                            GlobalStateData::on_change(cur, old);
                        },
                    )*
                }
            }

            pub fn config(&self, log: &Logger, sender: &Sender<FrontendMsg>){
                use DebugViewConfMsg::*;
                $(
                    {
                        let cur = &self.$up_ident;
                        let state = &cur.state;
                        let view = &cur.view;
                        if let Some(msg) = view.config(state){
                            match sender.try_send(FrontendMsg::DebugView($up_msg (msg))){
                                Ok(_) => {},
                                Err(TrySendError::Full(msg)) => warn!(log, "frontend to backend message queue is full! discarding msg: {msg:?}"),
                                Err(e) => warn!(log, "frontend to backend message queue error: {e:?}"),
                            }
                        }
                    }
                )*
            }
        }
    };
}

debug_views! {
    data registers, Registers, registers::Registers;
    update disassembly_view, DisassemblyView, "disassmebly-view", disassembly::DVDisasm;
    instance cartridge_view, "cartridge-view", cartridge::DVCartridge;
}
