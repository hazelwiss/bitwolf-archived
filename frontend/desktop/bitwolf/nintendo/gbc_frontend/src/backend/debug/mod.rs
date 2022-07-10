pub(crate) mod messages;

mod input;
mod state;
mod sync;

use crate::{backend::FrameBuffer, messages::FtoC, Audio};
use gbc_backend::{engines::interpreter, Builder, Core};
use state::State;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
    time::Duration,
};

type MsgQ = util::bdq::Bdq<FtoC, messages::CtoF>;

pub fn run(
    builder: Builder,
    running: Arc<AtomicBool>,
    mut msgq: MsgQ,
    input_recv: Receiver<interpreter::input::InputState>,
    fb: FrameBuffer,
    audio: Audio,
) {
    let mut backend = Box::new(Core::<interpreter::Interpreter>::new(builder));
    let mut state = State::default();
    let mut test_sample = 0.0f32;
    // Initial sync.
    sync::sync(&mut backend, &state, &mut msgq);
    while running.load(Ordering::Relaxed) {
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
                    messages::recv(emu, &mut state, &mut msgq);
                    // Receives input from input queue.
                    input::recv(emu, &input_recv);
                    // Presents frame.
                    fb.get().write().data = frame.data;
                    // TEMP
                    audio.push_sample(test_sample.sin());
                    test_sample += 0.012;
                    // wait a little. TODO: sync with sound!
                    std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
                },
            )
        } else {
            messages::recv(&mut backend, &mut state, &mut msgq);
            interpreter::frame(&mut backend, |frame, _| {
                fb.get().write().data = frame.data;
            });
            std::thread::sleep(Duration::from_millis(1000 / 60));
        }
    }
}
