use crate::{backend::FrameBuffer, AudioSampler};
use gbc_backend::{engines::interpreter, Builder, Core};

pub fn _run(builder: Builder, fb: FrameBuffer, sampler: AudioSampler) {
    let mut backend = Core::<interpreter::Interpreter>::new(builder, Box::new(sampler));
    loop {
        interpreter::run_until_frame(
            &mut backend,
            None,
            #[inline(always)]
            |frame, _| {
                // Presents frame.
                fb.get().write().data = frame.data;
            },
        );
    }
}
