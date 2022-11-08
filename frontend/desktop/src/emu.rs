use crate::debug_views::{self, DVEmuStateMsg, DVStateMsg};
use bitwolf_core::{debug, engine::Engine, interpreter, Core};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use util::log::Logger;

/// Messages sent by the emu thread.
#[derive(Debug)]
pub enum EmuMsg {
    DebugView(DVStateMsg),
}

/// Messages sent by the frontend.
#[derive(Debug)]
pub enum FrontendMsg {
    DebugView(DVEmuStateMsg),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub running: Arc<AtomicBool>,
    pub stopped: Arc<AtomicBool>,
}

unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}

type Sender = crossbeam_channel::Sender<EmuMsg>;
type Receiver = crossbeam_channel::Receiver<FrontendMsg>;

fn dv_update<E: Engine>(
    core: &mut Core<E>,
    dv_conf: &mut debug_views::DVEmuState,
    sender: &Sender,
) {
    let conf = &dv_conf.disassembly_view;
    let start_adr = conf.start_adr;
    let mut vec = vec![];
    for i in 0..conf.line_cnt {
        vec.push(debug::disassembly::disassemble_arm9(
            core,
            start_adr.wrapping_add((4 * i) as u32),
        ))
    }
    let _ = sender.send(EmuMsg::DebugView(DVStateMsg::DisassemblyView(
        debug_views::disassembly::Local { disasm: vec },
    )));

    let _ = sender.send(EmuMsg::DebugView(DVStateMsg::Registers(
        debug_views::registers::State {
            pc: core.arm9.registers.get_pc(),
        },
    )));
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    core_builder: bitwolf_core::CoreBuilder,
    #[allow(unused)] log: Logger,
    shared_state: SharedState,
    sender: Sender,
    recver: Receiver,
) {
    let mut core = core_builder.build::<bitwolf_core::Interpreter>();
    let mut dv_conf = debug_views::DVEmuState::default();

    let _ = sender.send(EmuMsg::DebugView(DVStateMsg::Cartridge(
        debug_views::cartridge::State {
            cartridge_header: debug::cartridge_info::cartridge_header(&core),
        },
    )));

    while shared_state.running.load(Ordering::Relaxed) {
        let mut arm9_step = 0;
        if let Ok(recv) = recver.try_recv() {
            match recv {
                FrontendMsg::DebugView(conf) => match conf {
                    DVEmuStateMsg::DisassemblyView(conf) => dv_conf.disassembly_view = conf,
                    DVEmuStateMsg::Cartridge(cart) => dv_conf.cartridge_view = cart,
                    DVEmuStateMsg::Control(control) => {
                        arm9_step = if let Some(val) = control.arm9_step {
                            val
                        } else {
                            0
                        };
                    }
                },
            }
        }
        if shared_state.stopped.load(Ordering::Relaxed) {
            for _ in 0..arm9_step {
                interpreter::arm9::step(&mut core);
            }
        } else {
            // TODO: run like normal.
        }
        dv_update(&mut core, &mut dv_conf, &sender);
    }
}
