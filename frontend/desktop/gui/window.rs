use imgui::{Io, Ui};
use imgui_wgpu::RendererConfig;
use imgui_winit_support::HiDpiMode;
use pollster::block_on;
use std::mem::ManuallyDrop;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop as WinitEventLoop};
use winit::window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder};

struct Gfx {
    surface: wgpu::Surface,
    surface_conf: wgpu::SurfaceConfiguration,
    queue: wgpu::Queue,
    device: wgpu::Device,
    texture_format: wgpu::TextureFormat,
    present_mode: wgpu::PresentMode,
    renderer: imgui_wgpu::Renderer,
}

impl Gfx {
    fn new(handle: &WinitWindow, imgui_ctx: &mut imgui::Context) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(handle) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("unable to create wgpu adapter");
        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .expect("unable to create device and queue");
        let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let present_mode = wgpu::PresentMode::AutoVsync;
        let window_size = handle.inner_size();
        let surface_conf = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: window_size.width,
            height: window_size.height,
            present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_conf);
        let renderer = imgui_wgpu::Renderer::new(
            imgui_ctx,
            &device,
            &queue,
            RendererConfig {
                texture_format,
                ..Default::default()
            },
        );
        Self {
            surface,
            surface_conf,
            queue,
            device,
            texture_format,
            present_mode,
            renderer,
        }
    }
}

pub struct Window {
    handle: WinitWindow,
    gfx: Gfx,
    clear_colour: wgpu::Color,
}

pub struct Builder {
    clear_colour: wgpu::Color,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            clear_colour: wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }

    pub fn build<T>(
        self,
        event_loop: &WinitEventLoop<T>,
        imgui_ctx: &mut imgui::Context,
    ) -> Result<Window, winit::error::OsError> {
        let winit_handle = WinitWindowBuilder::new().build(event_loop)?;
        let gfx = Gfx::new(&winit_handle, imgui_ctx);
        let Self { clear_colour } = self;
        Ok(Window {
            handle: winit_handle,
            gfx,
            clear_colour,
        })
    }
}

pub fn run<
    T,
    State: 'static,
    Update: FnMut(&mut State, &mut Ui, &Io, &mut ControlFlow) + 'static,
    OnInput: FnMut(&mut State) + 'static,
    OnExit: FnOnce(State) + 'static,
>(
    event_loop: WinitEventLoop<T>,
    window: Window,
    state: State,
    mut imgui: imgui::Context,
    mut update: Update,
    mut on_input: OnInput,
    on_exit: OnExit,
) {
    let mut state = ManuallyDrop::new(state);
    let mut on_exit = ManuallyDrop::new(on_exit);
    let Window {
        handle: window,
        mut gfx,
        clear_colour,
    } = window;
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match &event {
            Event::WindowEvent { window_id, event } if *window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    info!("close requested");
                    control_flow.set_exit()
                }
                WindowEvent::Resized(_) => {
                    let size = window.inner_size();
                    if size.width > 0 && size.height > 0 {
                        gfx.surface_conf.width = size.width;
                        gfx.surface_conf.height = size.height;
                        gfx.surface.configure(&gfx.device, &gfx.surface_conf);
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(id) if *id == window.id() => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
                let frame = gfx
                    .surface
                    .get_current_texture()
                    .expect("unable to create frame");
                let mut encoder =
                    gfx.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("command encoder"),
                        });
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some("texture view"),
                    ..Default::default()
                });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("render pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_colour),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("failed to prepare frame");
                    let io = unsafe { &*(imgui.io() as *const Io) };
                    let ui = imgui.frame();
                    update(&mut state, ui, io, control_flow);
                    let frame = imgui.render();
                    gfx.renderer
                        .render(frame, &gfx.queue, &gfx.device, &mut rpass)
                        .expect("failed to render imgui frame")
                }
                gfx.queue.submit(Some(encoder.finish()));
                frame.present()
            }
            Event::LoopDestroyed => {
                info!("event loop destroyed");
                unsafe { (ManuallyDrop::take(&mut on_exit))(ManuallyDrop::take(&mut state)) };
                return;
            }
            _ => {}
        }
        platform.handle_event(imgui.io_mut(), &window, &event)
    });
}
