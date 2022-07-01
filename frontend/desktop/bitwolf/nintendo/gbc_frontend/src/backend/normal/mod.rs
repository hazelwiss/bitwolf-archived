use crate::backend::FrameBuffer;
use gbc_backend::{engines::interpreter, Builder, Core};

pub fn _run(builder: Builder, fb: FrameBuffer) {
    let mut backend = Core::<interpreter::Interpreter>::new(builder);
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
