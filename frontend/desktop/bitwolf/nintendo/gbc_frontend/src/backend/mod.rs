pub mod messages;

mod state;
mod sync;

use crate::messages::FtoC;
use common_frontend::framebuffer;
use gbc_backend::{engines::interpreter, Builder, Core};
use messages::CtoF;
use util::bdq::Bdq;

type FrameBuffer = framebuffer::access::AccessW<gbc_backend::Texture>;
type MsgQ = Bdq<FtoC, CtoF>;

pub fn run(builder: Builder, mut bdq: MsgQ, fb: FrameBuffer) {
    let mut backend = Core::<interpreter::Interpreter>::new(builder);
    let mut state = state::State::default();
    loop {
        interpreter::run_until_frame(
            &mut backend,
            #[inline(always)]
            |frame, emu| {
                // Receive from message queue.
                messages::msgq_recv(emu, &mut state, &mut bdq);
                // Sync with frontend.
                sync::sync(emu, &mut bdq);
                // Presents frame.
                fb.get().write().data = frame.data;
            },
        );
    }
}
