use crate::{
    common::{
        self,
        demsgq::{make_pair, Demsgq},
    },
    cores::nds,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use winit::platform::unix::x11::ffi::Atom;

pub enum MsgCtoF {
    #[cfg(debug_assertions)]
    Debug(),
}

pub enum MsgFtoC {
    #[cfg(debug_assertions)]
    Debug(),
}

pub struct CoreState {
    running: Arc<AtomicBool>,
    msgq: Demsgq<MsgFtoC, MsgCtoF>,
}

impl CoreState {}

impl Drop for CoreState {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

pub struct ThreadedBuilder {
    pub msgq_capacity: usize,
    pub core_builder: nds_core::core::Builder,
}

impl ThreadedBuilder {
    pub fn build<E: nds_core::engine::Engine>(self) -> CoreState {
        let (msgq_c, msgq_f) = make_pair(self.msgq_capacity);
        let running_f = Arc::new(AtomicBool::new(true));
        let running_c = running_f.clone();
        std::thread::spawn(move || {
            // Core update loop.
            let core = self.core_builder.build::<E>();
            while running_c.load(Ordering::Relaxed) {}
        });
        CoreState {
            running: running_f,
            msgq: msgq_f,
        }
    }
}
