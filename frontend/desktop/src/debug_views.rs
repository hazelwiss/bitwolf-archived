#![allow(dead_code)]

pub mod cartridge;
pub mod control;
pub mod disassembly;
pub mod registers;

use crate::{emu::FrontendMsg, gui::window::Window};
use core::ops::{Deref, DerefMut};
use crossbeam_channel::{Sender, TrySendError};
use imgui::{Io, Ui};
use util::log::{warn, Logger};

pub trait DynamicDV: Default {
    type Local: Default;
    type Emu: Default;

    fn draw(
        &mut self,
        state: &mut Self::Local,
        global_state: &GlobalState,
        window: &mut Window,
        ui: &Ui,
        io: &Io,
    );

    fn has_menu_bar(&self) -> bool;

    fn on_change(&mut self, old: Self::Local, new: &mut Self::Local);

    fn emu_update(&self) -> Option<Self::Emu>;
}

pub trait StaticDV: Default {
    type Emu: Default;

    fn draw(&mut self, global_state: &GlobalState, window: &mut Window, ui: &Ui, io: &Io);

    fn has_menu_bar(&self) -> bool;

    fn emu_update(&mut self) -> Option<Self::Emu>;
}

pub trait GlobalStateData: Default {
    type State: Default;

    fn on_change(&mut self, old: Self::State);

    fn set_state(&mut self, new: Self::State);

    fn get_state(&self) -> Self::State;
}

#[derive(Default)]
struct DynamicViewHandler<T: DynamicDV> {
    view: T,
    state: T::Local,
    open: bool,
}

impl<T: DynamicDV> DynamicViewHandler<T> {
    fn from_state(state: T::Local) -> Self {
        Self {
            state,
            view: Default::default(),
            open: false,
        }
    }
}

impl<T: DynamicDV> Deref for DynamicViewHandler<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<T: DynamicDV> DerefMut for DynamicViewHandler<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

#[derive(Default)]
struct StaticViewHandler<T: StaticDV> {
    view: T,
    open: bool,
}

impl<T: StaticDV> Deref for StaticViewHandler<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<T: StaticDV> DerefMut for StaticViewHandler<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

macro_rules! debug_views {
    (
        $(data $da_ident:ident, $da_msg:ident, $da_ty:ty);*;
        $(dynamic $up_ident:ident, $up_msg:ident, $up_print:literal, $up_ty:ty);*;
        $(static $st_ident:ident, $st_msg:ident, $st_print:literal, $st_ty:ty);*
        $(;)?
    ) => {
        /// The DV global state.
        #[derive(Default)]
        pub struct GlobalState{
            $(
                $da_ident: $da_ty
            ),*
        }

        /// Sent from the core to update the DV state.
        #[derive(Debug)]
        pub enum DVStateMsg{
            $(
                $up_msg (<$up_ty as DynamicDV>::Local),
            )*
            $(
                $da_msg (<$da_ty as GlobalStateData>::State),
            )*
        }

        /// Sent as ways to instruct or control the core.
        #[derive(Debug)]
        pub enum DVEmuStateMsg{
            $(
                $up_msg (<$up_ty as DynamicDV>::Emu),
            )*
            $(
                $st_msg (<$st_ty as StaticDV>::Emu),
            )*
        }

        /// The debug view state intended to be usedo n the emulator end.
        #[derive(Default)]
        pub struct DVEmuState{
            $(
                pub $up_ident: <$up_ty as DynamicDV>::Emu,
            )*
            $(
                pub $st_ident: <$st_ty as StaticDV>::Emu,
            )*
        }

        impl DVEmuState{
            #[inline]
            pub fn update(&mut self, msg: DVEmuStateMsg){
                use DVEmuStateMsg::*;
                match msg{
                    $(
                        $up_msg (conf) => self.$up_ident = conf,
                    )*
                    $(
                        $st_msg (cond) => self.$st_ident = cond,
                    )*
                }
            }
        }

        #[derive(Default)]
        pub struct DebugViews {
            $(
                $up_ident: DynamicViewHandler<$up_ty>,
            )*
            $(
                $st_ident: StaticViewHandler<$st_ty>,
            )*
            global_state: GlobalState,
        }

        impl DebugViews{
            pub fn draw(&mut self, window: &mut Window, ui: &Ui, io: &Io){
                $(
                    {
                        let cur = &mut self.$up_ident;
                        let open = &mut cur.open;
                        let view = &mut cur.view;
                        let state = &mut cur.state;
                        if *open{
                            imgui::Window::new($up_print).opened(open).menu_bar(view.has_menu_bar()).build(ui, ||{
                                view.draw(state, &self.global_state, window, ui, io);
                            });
                        }
                    }
                )*
                $(
                     {
                         let cur = &mut self.$st_ident;
                         let open = &mut cur.open;
                         let view = &mut cur.view;
                         if *open{
                             imgui::Window::new($st_print).opened(open).menu_bar(view.has_menu_bar()).build(ui, ||{
                                view.draw(&self.global_state, window, ui, io);
                             });
                         }
                     }
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
                            let cur = &mut self.$st_ident;
                            let name = $st_print;
                            ui.checkbox(name, &mut cur.open);
                        }
                    )*
                });
            }

            pub fn update_state(&mut self, state: DVStateMsg){
                use DVStateMsg::*;
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

            pub fn update_emu_state(&mut self, log: &Logger, sender: &Sender<FrontendMsg>){
                use DVEmuStateMsg::*;
                macro_rules! update{
                    ($ident:ident, $msg:ident) => {
                        {
                            let cur = &mut self.$ident;
                            //let state = &cur.state;
                            let view = &mut cur.view;
                            if let Some(msg) = view.emu_update(){
                                match sender.try_send(FrontendMsg::DebugView($msg (msg))){
                                    Ok(_) => {},
                                    Err(TrySendError::Full(msg)) => warn!(log, "frontend to backend message queue is full! discarding msg: {msg:?}"),
                                    Err(e) => warn!(log, "frontend to backend message queue error: {e:?}"),
                                }
                            }
                        }
                    };
                }
                $(
                    update!($up_ident, $up_msg);
                )*
                $(
                    update!($st_ident, $st_msg);
                )*

            }
        }
    };
}

debug_views! {
    data registers, Registers, registers::Registers;
    dynamic disassembly_view, DisassemblyView, "disassmebly-view", disassembly::DVDisasm;
    dynamic cartridge_view, Cartridge, "cartridge-view", cartridge::DVCartridge;
    static control_view, Control, "control-view", control::Control;
}
