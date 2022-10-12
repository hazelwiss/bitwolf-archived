#![allow(dead_code)]

use imgui::Context as Imgui;
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pollster::block_on;
use std::time::Instant;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
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
    pub clear_colour: wgpu::Color,
    platform: WinitPlatform,
    winit_window: WinitWindow,
}

impl Window {
    pub fn new(imgui: &mut Imgui, window: WinitWindow, clear_colour: wgpu::Color) -> Self {
        let mut platform = WinitPlatform::init(imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
        let gfx = GfxContext::from_winit_window(&window, imgui).expect("unable to create ");
        Self {
            winit_window: window,
            gfx,
            clear_colour,
            platform,
        }
    }

    #[inline]
    pub fn resize_to_inner(&mut self) {
        let size = self.winit_window.inner_size();
        self.resize(size);
    }

    #[inline]
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.update_size(size);
        }
    }

    #[inline]
    fn update_size(&self, size: PhysicalSize<u32>) {
        self.gfx.resize(size);
    }
}

pub struct WindowBuilder<T: 'static> {
    pub imgui: Imgui,
    pub window: Window,
    pub event_loop: EventLoop<T>,
}

impl<T: 'static> WindowBuilder<T> {
    pub fn run<S: 'static>(
        self,
        mut state: S,
        mut f_event: impl FnMut(&mut S, Event<T>, &mut Window, &mut Imgui) + 'static,
        mut f_frame: impl FnMut(&mut S, &imgui::Ui, &mut Window) + 'static,
    ) -> ! {
        let mut window = self.window;
        let mut imgui = self.imgui;

        let mut last_frame = Instant::now();
        self.event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Poll;
            window
                .platform
                .handle_event(imgui.io_mut(), &window.winit_window, &event);
            match &event {
                Event::WindowEvent { event, .. } => match &event {
                    WindowEvent::Resized(_) => window.resize_to_inner(),
                    WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::MainEventsCleared => window.winit_window.request_redraw(),
                Event::RedrawEventsCleared => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    let frame = window
                        .gfx
                        .surface
                        .get_current_texture()
                        .expect("unabel to create frame");

                    let mut encoder = window
                        .gfx
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
                                    load: wgpu::LoadOp::Clear(window.clear_colour),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                        window
                            .platform
                            .prepare_frame(imgui.io_mut(), &window.winit_window)
                            .expect("Failed to prepare frame");
                        let ui = imgui.frame();
                        f_frame(&mut state, &ui, &mut window);
                        let frame = ui.render();
                        window
                            .gfx
                            .renderer
                            .render(frame, &window.gfx.queue, &window.gfx.device, &mut rpass)
                            .expect("Failed to render imgui frame.");
                    }
                    window.gfx.queue.submit(Some(encoder.finish()));
                    frame.present()
                }
                Event::LoopDestroyed => *flow = ControlFlow::Exit,
                _ => {}
            }
            f_event(&mut state, event, &mut window, &mut imgui);
        });
    }
}
