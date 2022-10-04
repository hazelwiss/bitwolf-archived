#![allow(dead_code)]

pub mod imgui_ctx;
pub mod window;

use pollster::block_on;
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
pub enum GfxError {}

pub struct GfxContext {
    pub(super) surface: wgpu::Surface,
    pub(super) queue: wgpu::Queue,
    pub(super) device: wgpu::Device,
    pub(super) texture_format: wgpu::TextureFormat,
    pub(super) present_mode: wgpu::PresentMode,
}

pub type Result<T> = core::result::Result<T, GfxError>;

pub struct TextureId();

impl GfxContext {
    pub(super) fn from_winit_window(window: &Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Unable to create WGPU adapter.");
        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .expect("WGPU was unable to create device and queue.");
        let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let present_mode = wgpu::PresentMode::AutoVsync;
        let window_size = window.inner_size();
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: present_mode,
        };
        surface.configure(&device, &surface_desc);
        Ok(Self {
            surface,
            queue,
            device,
            texture_format,
            present_mode,
        })
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.texture_format,
            width: size.width,
            height: size.height,
            present_mode: self.present_mode,
        };
        self.surface.configure(&self.device, &surface_desc);
    }

    pub fn new_texture(&mut self) -> Result<TextureId> {
        todo!()
    }

    pub fn modify_texture(&mut self, _id: TextureId) {
        todo!()
    }

    pub fn destroy_texture(&mut self, _id: TextureId) {
        todo!()
    }
}
