pub mod demsgq;
pub mod windows;

use crate::window_loop::ImguiCtx;

pub trait CoreFrontend {
    #[cfg(debug_assertions)]
    fn update_panels(&mut self, run_ctx: &mut ImguiCtx);

    fn sync_core(&mut self);
}
