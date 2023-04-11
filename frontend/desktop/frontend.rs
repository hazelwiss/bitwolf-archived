mod nds_impl;

use crossbeam::{Receiver, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const CHANNEL_BUF_SIZE: usize = 64;

macro_rules! impl_frontend {
    (
        $(
            $variant_ident:ident { interp: $core_interp:ty, jit: $core_jit:ty } : $mod:ident {
                // Runs the interpreter on the core. Potentially generates a new frame to be presented.
                core_interp_run_fn: $core_interp_run_fn:path,
                // Creates the DebugState from the core.
                core_mk_debug_state_fn: $core_mk_debug_state_fn:path,
                // Update function called on the frontend.
                interp_update_fn: $interp_update_fn:path,
                // Setup function.
                setup_fn: $setup_fn:path,
                // Debug global state.
                debug_global_state: $debug_global_state:ty,
                // GUI elements.
                gui : [
                    setup: [
                        $($gui_setup_ident:ident [$gui_setup_print:literal]: $gui_setup:ty,)*
                    ]
                    update: [
                        $($gui_ident:ident [$gui_print:literal]: $gui:ty,)*
                    ]
                    functional: [
                        $($gui_func_ident:ident [$gui_func_print:literal]: $gui_func:ty,)*
                    ]
                ],
            };
        )*
    ) => {
        pub enum Frontend {
            None,
            $(
                $variant_ident($mod::State),
            )*
        }

        pub enum Core {
            $(
                $variant_ident,
            )*
        }

        impl Frontend {
            pub fn halt(&self) {
                match self {
                    $(Self::$variant_ident(var) => $mod::halt(var),)*
                    Self::None => {}
                }
            }

            pub fn resume(&self) {
                match self {
                    $(Self::$variant_ident(var) => $mod::resume(var),)*
                    Self::None => {}
                }
            }

            pub fn update(&mut self, ui: &imgui::Ui, io: &imgui::Io) {
                match self {
                    $(Self::$variant_ident(var) => $mod::update(var, ui, io),)*
                    Self::None => {},
                }
            }

            pub fn input(&mut self) {
                match self {
                    $(Self::$variant_ident(var) => $mod::input(var),)*
                    Self::None => {},
                }
            }

            pub fn draw_menu_debug(&mut self, ui: &imgui::Ui) {
                match self {
                    $(Self::$variant_ident(var) => $mod::draw_menu_debug(var, ui),)*
                    Self::None => {}
                }
            }

            pub fn exit(mut self) {
                todo!()
            }
        }
        $(
            pub mod $mod {
                use super::*;

                pub type Interp = $core_interp;
                pub type Jit = $core_jit;
                pub type DebugGlobalState = $debug_global_state;

                pub trait DebugGuiFunctional: Default {
                    fn draw(state: &mut State, ui: &imgui::Ui, io: &imgui::Io);
                }

                pub trait DebugGuiSetup {
                    fn setup_interp(core: &Interp) -> Self;

                    fn setup_jit(core: &Jit) -> Self;

                    fn draw(&mut self, ui: &imgui::Ui, io: &imgui::Io);
                }

                pub trait DebugGui: Default {
                    type State: Default;
                    type Conf: Default + Clone;

                    /// `self` can hold temporary data during drawing.
                    /// `state` holds information collected from the core.
                    /// `conf` holds information for the core on how to collect information for the debug interface.
                    fn draw(&mut self, ui: &imgui::Ui, io: &imgui::Io, global_state: &DebugGlobalState, state: &Self::State, conf: &mut Self::Conf);
                }

                pub enum Engine {
                    Interp(Interp),
                    Jit(Jit),
                }

                pub struct State {
                    debug: Debug,
                    debug_enabled: DebugEnabled,
                    debug_state: Box<DebugState>,
                    debug_conf: DebugConf,
                    pub sender: Sender<FtoC>,
                    receiver: Receiver<CtoF>,
                    pub running: Arc<AtomicBool>,
                    pub halted: Arc<AtomicBool>,
                }

                impl State {
                    pub(super) fn new(mut core: Engine) -> Self {
                        let (s0, r1) = crossbeam::bounded(CHANNEL_BUF_SIZE);
                        let (s1, r0) = crossbeam::bounded(CHANNEL_BUF_SIZE);
                        let running = Arc::new(AtomicBool::new(true));
                        let halted = Arc::new(AtomicBool::new(true));
                        let mut new = Self {
                            debug: Debug {
                                $($gui_ident: Default::default(),)*
                                $($gui_func_ident: Default::default(),)*
                                $(
                                    $gui_setup_ident: match &core {
                                        Engine::Interp(core) => <$gui_setup as DebugGuiSetup>::setup_interp(&core) ,
                                        Engine::Jit(core) => <$gui_setup as DebugGuiSetup>::setup_jit(&core) ,
                                    }
                                )*
                            },
                            debug_enabled: DebugEnabled::default(),
                            debug_state: Box::new(DebugState::default()),
                            debug_conf: DebugConf::default(),
                            sender: s0,
                            receiver: r0,
                            running: running.clone(),
                            halted: halted.clone(),
                        };
                        std::thread::spawn(move || match core {
                            Engine::Interp(core) => threaded_interp_run_generic(core, s1, r1, running, halted),
                            Engine::Jit(_) => unimplemented!(),
                        });
                        $setup_fn(&mut new);
                        new
                    }
                }

                pub struct Debug {
                    $(
                        $gui_ident: $gui,
                    )*
                    $(
                        $gui_setup_ident: $gui_setup,
                    )*
                    $(
                        $gui_func_ident: $gui_func,
                    )*
                }

                #[derive(Default)]
                pub struct DebugEnabled {
                    $(
                        $gui_ident: bool,
                    )*
                    $(
                        $gui_setup_ident: bool,
                    )*
                    $(
                        $gui_func_ident: bool,
                    )*
                }

                #[derive(Default)]
                pub struct DebugState {
                    pub(super) global_debug_state: DebugGlobalState,
                    $(
                        pub(super) $gui_ident: <$gui as DebugGui>::State,
                    )*
                }

                #[derive(Default, Clone)]
                pub struct DebugConf {
                    $(
                        pub(super) $gui_ident: <$gui as DebugGui>::Conf,
                    )*
                }

                /// Core to frontend messages.
                pub(super) enum CtoF {
                    Step,
                    DebugState(Box<DebugState>),
                }

                /// Frontend to core messages.
                pub enum FtoC {
                    Step,
                    DebugConf(Box<DebugConf>),
                }

                pub(super) fn halt(state: &State) {
                    state.halted.store(true, Ordering::Relaxed);
                }

                pub(super) fn resume(state: &State) {
                    state.halted.store(false, Ordering::Relaxed);
                }

                pub(super) fn update(state: &mut State, ui: &imgui::Ui, io: &imgui::Io) {
                    $interp_update_fn(state);
                    let _ = state.sender.try_send(FtoC::DebugConf(Box::new(state.debug_conf.clone())));
                    match state.receiver.try_recv() {
                        Ok(CtoF::DebugState(recv)) => state.debug_state = recv,
                        _ => {}
                    }
                    draw_debug(state, ui, io);
                }

                pub(super) fn input(state: &mut State) {}

                pub(super) fn exit(mut state: State) {}

                pub(super) fn draw_menu_debug(state: &mut State, ui: &imgui::Ui) {
                    let _ = state.halted.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |mut val| {
                        ui.checkbox("halted", &mut val);
                        Some(val)
                    });
                    ui.menu("debug views", ||{
                        $(
                            ui.checkbox($gui_print, &mut state.debug_enabled.$gui_ident);
                        )*
                        $(
                            ui.checkbox($gui_setup_print, &mut state.debug_enabled.$gui_setup_ident);
                        )*
                        $(
                            ui.checkbox($gui_func_print, &mut state.debug_enabled.$gui_func_ident);
                        )*
                    });
                }

                fn draw_debug(state: &mut State, ui: &imgui::Ui, io: &imgui::Io) {
                    $(
                        if state.debug_enabled.$gui_ident {
                            ui.window($gui_print).build(|| {
                                <$gui as DebugGui>::draw(
                                    &mut state.debug.$gui_ident,
                                    ui,
                                    io,
                                    &state.debug_state.global_debug_state,
                                    &state.debug_state.$gui_ident,
                                    &mut state.debug_conf.$gui_ident
                                );
                            });
                        }
                    )*
                    $(
                        if state.debug_enabled.$gui_setup_ident {
                            ui.window($gui_setup_print).build(|| {
                                <$gui_setup as DebugGuiSetup>::draw(&mut state.debug.$gui_setup_ident, ui, io);
                            });
                        }
                    )*
                    $(
                        if state.debug_enabled.$gui_func_ident {
                            ui.window($gui_func_print).build(|| {
                                <$gui_func as DebugGuiFunctional>::draw(state, ui, io);
                            });
                        }
                    )*
                }

                /// Threaded generic core run function.
                fn threaded_interp_run_generic(
                    mut core: $core_interp,
                    sender: Sender<CtoF>,
                    receiver: Receiver<FtoC>,
                    running: Arc<AtomicBool>,
                    halted: Arc<AtomicBool>)
                {
                    let mut conf = Box::new(DebugConf::default());
                    while running.load(Ordering::Relaxed) {
                        match receiver.try_recv() {
                            Ok(FtoC::DebugConf(recv)) => conf = recv,
                            _ => {}
                        }
                        let debug_state = $core_mk_debug_state_fn(&mut core, &conf);
                        let _ = sender.try_send(CtoF::DebugState(Box::new(debug_state)));
                        while halted.load(Ordering::Relaxed) {}
                        $core_interp_run_fn(&mut core);
                    }
                }
            }
        )*
    };
}

use crate::gui;

impl_frontend! {
    NDS{ interp: ::nds::Core<::nds::Interpreter>, jit:() } : nds {
        core_interp_run_fn: nds_impl::core_interp_run,
        core_mk_debug_state_fn: nds_impl::core_mk_debug_state,
        interp_update_fn: nds_impl::interp_update,
        setup_fn: nds_impl::setup,
        debug_global_state: nds_impl::DebugGlobalState,
        gui: [
            setup: [
                metadata ["metadata"]: gui::debug::nds::Metadata,
            ]
            update: [
                disasm ["disassembly"]: gui::debug::nds::Disasm,
            ]
            functional: [
                control ["control"]: gui::debug::nds::Control,
            ]
        ],
    };
}

impl Frontend {
    pub fn none() -> Self {
        Self::None
    }

    pub fn nds(rom: Box<[u8]>) -> Self {
        Self::NDS(nds::State::new(nds::Engine::Interp(::nds::Core::new(rom))))
    }

    pub fn swap(&mut self) {}
}
