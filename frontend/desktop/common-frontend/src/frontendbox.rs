use crate::Frontend;
use imgui::WGPUContext;
use std::ops::{Deref, DerefMut};

pub struct FrontendBox {
    inner: Box<dyn Frontend>,
}

impl FrontendBox {
    pub fn new(frontend: impl Frontend + 'static) -> Self {
        Self {
            inner: Box::new(frontend),
        }
    }

    pub fn from_box(frontend: Box<dyn Frontend>) -> Self {
        Self { inner: frontend }
    }

    pub fn swap(&mut self, mut other: Box<dyn Frontend>, wgpu_ctx: &mut WGPUContext) {
        std::mem::swap(&mut other, &mut self.inner);
        other.destroy(wgpu_ctx);
    }
}

impl Deref for FrontendBox {
    type Target = dyn Frontend;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for FrontendBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}
