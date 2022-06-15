use crate::Frontend;
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

    pub fn swap(&mut self, other: Box<dyn Frontend>) {
        self.inner = other;
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
