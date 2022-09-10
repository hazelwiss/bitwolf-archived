#[cfg(feature = "debug-views")]
mod debug_msgs;
#[cfg(feature = "debug-views")]
mod debug_state;
mod interpreter;
mod jit;

use crate::common::{
    self,
    demsgq::{make_pair, Demsgq},
};
use nds_core::{core, engine::Engine, interpreter::Interpreter};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
};

use super::debug_views::{self, DebugViewMsg};

pub enum MsgCtoF {
    #[cfg(feature = "debug-views")]
    DebugView(debug_views::DebugViewMsg),
}

pub enum MsgFtoC {}

pub struct FState {
    pub running: Arc<AtomicBool>,
    pub msgq: Demsgq<MsgCtoF, MsgFtoC>,
    join_handle: Option<JoinHandle<()>>,
}

impl FState {}

impl Drop for FState {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        let join_handle = self.join_handle.take().unwrap();
        join_handle.join().expect("unable to join core thread");
    }
}

pub(crate) struct CState {
    running: Arc<AtomicBool>,
    msgq: Demsgq<MsgFtoC, MsgCtoF>,
}

impl CState {
    #[cfg(feature = "debug-views")]
    fn dbgview_msg(&self, msg: DebugViewMsg) {
        self.msgq.send(MsgCtoF::DebugView(msg));
    }
}

pub struct Builder {
    pub msgq_capacity: usize,
    pub core_builder: core::Builder,
}

impl Builder {
    pub fn build_interp(self) -> FState {
        let (msgq_f, msgq_c) = make_pair(self.msgq_capacity);
        let running_f = Arc::new(AtomicBool::new(true));
        let running_c = running_f.clone();
        let join_handle = std::thread::spawn(move || {
            interpreter::run(
                CState {
                    running: running_c,
                    msgq: msgq_c,
                },
                self.core_builder
                    .build()
                    .expect("Unable to build core (interpreter)"),
            );
        });
        FState {
            running: running_f,
            msgq: msgq_f,
            join_handle: Some(join_handle),
        }
    }
}
