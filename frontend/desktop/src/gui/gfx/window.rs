use super::GfxContext;
use std::ops::{Deref, DerefMut};
use winit::{dpi::PhysicalSize, window::Window as WinitWindow};

#[allow(dead_code)]
pub struct WindowGfx {
    pub gfx: GfxContext,
    pub window: WinitWindow,
    pub clear_colour: wgpu::Color,
}

impl Deref for WindowGfx {
    type Target = WinitWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for WindowGfx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl WindowGfx {
    pub fn new(window: WinitWindow, clear_colour: wgpu::Color) -> Self {
        let gfx = GfxContext::from_winit_window(&window).expect("unable to create ");
        Self {
            window,
            gfx,
            clear_colour,
        }
    }

    pub fn update_size(&self, size: PhysicalSize<u32>) {
        self.gfx.resize(size);
    }
}
