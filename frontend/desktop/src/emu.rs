use crate::debug_views::{self, DebugViewConfMsg, DebugViewMsg as DVMsg};
use bitwolf_core::debug;
use crossbeam_channel::{Receiver, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use util::log::{warn, Logger};

#[derive(Debug)]
pub enum EmuMsg {
    DebugView(DVMsg),
}

#[derive(Debug)]
pub enum FrontendMsg {
    DebugView(DebugViewConfMsg),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub running: Arc<AtomicBool>,
}

unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}

#[allow(clippy::too_many_arguments)]
pub fn run(
    core_builder: bitwolf_core::CoreBuilder,
    log: Logger,
    shared_state: SharedState,
    msg_send: Sender<EmuMsg>,
    msg_recv: Receiver<FrontendMsg>,
) {
    let mut core = core_builder.build();
    let mut dv_conf = debug_views::DebugViewsConfState::default();

    let _ = msg_send.send(EmuMsg::DebugView(DVMsg::Cartridge(
        debug_views::cartridge::State {
            cartridge_header: debug::cartridge_info::cartridge_header(&core),
        },
    )));

    while shared_state.running.load(Ordering::Relaxed) {
        while let Ok(msg) = msg_recv.try_recv() {
            match msg {
                FrontendMsg::DebugView(conf) => dv_conf.update(conf),
            }
        }
        let conf = &dv_conf.disassembly_view;
        let start_adr = conf.start_adr;
        let mut vec = vec![];
        for i in 0..conf.line_cnt {
            vec.push(debug::disassembly::disassemble_arm9(
                &mut core,
                start_adr.wrapping_add((4 * i) as u32),
            ))
        }
        let _ = msg_send.try_send(EmuMsg::DebugView(DVMsg::DisassemblyView(
            debug_views::disassembly::State { disasm: vec },
        )));

        let _ = msg_send.try_send(EmuMsg::DebugView(DVMsg::Registers(
            debug_views::registers::State {
                pc: core.arm9.registers.get_pc(),
            },
        )));
    }
}
