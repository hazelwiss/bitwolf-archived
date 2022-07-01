pub(crate) mod messages;

mod state;
mod sync;

use crate::{backend::FrameBuffer, messages::FtoC};
use gbc_backend::{engines::interpreter, Builder, Core};
use state::State;

type MsgQ = util::bdq::Bdq<FtoC, messages::CtoF>;

pub fn run(builder: Builder, mut msgq: MsgQ, fb: FrameBuffer) {
    let mut backend = Core::<interpreter::Interpreter>::new(builder);
    let mut state = State::default();
    // Initial sync.
    sync::sync(&mut backend, &state, &mut msgq);
    loop {
        if state.ctrl.running {
            if state
                .ctrl
                .break_points
                .contains(&interpreter::debug::registers::get_pc(&backend))
            {
                state.ctrl.running = false;
                sync::sync(&mut backend, &state, &mut msgq);
                break;
            }
            interpreter::step(&mut backend);
            interpreter::frame(
                &mut backend,
                #[inline(always)]
                |frame, emu| {
                    // Receive from message queue.
                    messages::msgq_recv(emu, &mut state, &mut msgq);
                    // Presents frame.
                    fb.get().write().data = frame.data;
                },
            )
        } else {
            messages::msgq_recv(&mut backend, &mut state, &mut msgq);
            interpreter::frame(&mut backend, |frame, _| {
                fb.get().write().data = frame.data;
            })
        }
    }
}
