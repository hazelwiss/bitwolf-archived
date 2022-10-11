use crate::debug_views::DebugViewMsg;
use crossbeam_channel::Sender;
use std::sync::{atomic::AtomicBool, Arc};
use util::log::Logger;

pub enum Message {
    DebugView(DebugViewMsg),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub running: Arc<AtomicBool>,
}

unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}

pub(super) fn run(
    core: bitwolf_core::Core,
    logger: Logger,
    shared_state: SharedState,
    msg_sender: Sender<Message>,
) {
    loop {}
}
