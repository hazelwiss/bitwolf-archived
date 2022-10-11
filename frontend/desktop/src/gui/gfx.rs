#![allow(dead_code)]

use imgui::{FontConfig, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pollster::block_on;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window as WinitWindow,
};

#[derive(Debug)]
pub enum GfxError {}

pub type Result<T> = core::result::Result<T, GfxError>;

pub struct GfxContext {
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    device: wgpu::Device,
    texture_format: wgpu::TextureFormat,
    present_mode: wgpu::PresentMode,
    renderer: Renderer,
}

impl GfxContext {
    pub(super) fn from_winit_window(
        window: &WinitWindow,
        ctx: &mut imgui::Context,
    ) -> Result<Self> {
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
            present_mode,
        };
        surface.configure(&device, &surface_desc);

        let renderer = Renderer::new(
            ctx,
            &device,
            &queue,
            RendererConfig {
                texture_format,
                ..Default::default()
            },
        );

        Ok(Self {
            surface,
            queue,
            device,
            texture_format,
            present_mode,
            renderer,
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
}

#[allow(dead_code)]
pub struct Window {
    pub gfx: GfxContext,
    pub imgui: imgui::Context,
    platform: WinitPlatform,
    window: WinitWindow,
    clear_colour: wgpu::Color,
}

impl Deref for Window {
    type Target = WinitWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl Window {
    pub fn new(window: WinitWindow, clear_colour: wgpu::Color) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(Some(PathBuf::from("imgui.ini")));
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
        let hpdi_factor = window.scale_factor();
        imgui.io_mut().font_global_scale = (1.0 / hpdi_factor) as f32;
        let font_size = (13.0 * hpdi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);
        let gfx = GfxContext::from_winit_window(&window, &mut imgui).expect("unable to create ");
        Self {
            window,
            gfx,
            clear_colour,
            imgui,
            platform,
        }
    }

    pub fn draw<F>(&mut self, mut f: F)
    where
        F: FnMut(&imgui::Ui, &mut GfxContext),
    {
        let Self {
            window,
            imgui,
            gfx,
            platform,
            clear_colour,
        } = self;
        let frame = gfx
            .surface
            .get_current_texture()
            .expect("unabel to create frame");

        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(*clear_colour),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            platform
                .prepare_frame(imgui.io_mut(), window)
                .expect("Failed to prepare frame");
            let ui = self.imgui.frame();
            f(&ui, gfx);
            let frame = ui.render();
            gfx.renderer
                .render(frame, &gfx.queue, &gfx.device, &mut rpass)
                .expect("Failed to render imgui frame.");
        }
        gfx.queue.submit(Some(encoder.finish()));
        frame.present()
    }

    pub fn handle_event<T>(&mut self, event: Event<T>, flow: &mut ControlFlow) {
        self.platform
            .handle_event(self.imgui.io_mut(), &self.window, &event);
        #[allow(clippy::single_match)]
        match &event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_) => self.resize_to_inner(),
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn resize_to_inner(&mut self) {
        let size = self.window.inner_size();
        self.resize(size);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.update_size(size);
        }
    }

    fn update_size(&self, size: PhysicalSize<u32>) {
        self.gfx.resize(size);
    }
}
