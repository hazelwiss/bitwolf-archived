use imgui::{FontSource, Ui};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pollster::block_on;
use std::time::Instant;
use util::colour::BGRA;
use util::memory::to_byte_slice;
use wgpu::{FilterMode, SamplerDescriptor};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const PRESENT_MODE: wgpu::PresentMode = wgpu::PresentMode::Fifo;

pub struct DrawContext<'a> {
    ui: &'a Ui<'a>,
    wgpu_ctx: &'a mut WGPUContext,
}

impl<'a> DrawContext<'a> {
    pub fn ui(&self) -> &'a Ui<'a> {
        self.ui
    }

    pub fn resources(&mut self) -> &mut WGPUContext {
        self.wgpu_ctx
    }
}

pub struct WGPUImguiContext {
    pub imgui: imgui::Context,
    pub wgpu_ctx: WGPUContext,
}

pub struct WGPUContext {
    window: Window,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    device: wgpu::Device,
    platform: WinitPlatform,
    renderer: Renderer,
    clear_color: wgpu::Color,
}

impl WGPUImguiContext {
    pub fn create_imgui_window<T>(event_loop: &EventLoop<T>, clear_color: wgpu::Color) -> Self {
        /* Creates window. */
        let window_builder = WindowBuilder::new().with_maximized(true);
        let window = window_builder.build(&event_loop).unwrap();
        let window_size = window.inner_size();

        /* Creates the wgpu rendering context. */
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PRESENT_MODE,
        };
        surface.configure(&device, &surface_desc);

        /* Creates the imgui context and attaches it to the correct window. */
        let mut imgui = imgui::Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
        imgui.set_ini_filename(Some(std::path::PathBuf::from("imgui.ini"))); //  Disables ini file creation and loading.
        let hidpi_factor = window.scale_factor();
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        /* Attach imgui with the wgpu backend. */
        let renderer_config = RendererConfig {
            texture_format: surface_desc.format,
            ..Default::default()
        };
        let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);
        Self {
            imgui,
            wgpu_ctx: WGPUContext {
                window,
                surface,
                queue,
                device,
                platform,
                renderer,
                clear_color,
            },
        }
    }

    pub fn handle_events<T, F, RunCtx>(
        &mut self,
        event: &Event<T>,
        last_frame: &mut Instant,
        ctx: &mut RunCtx,
        f: &mut F,
    ) where
        F: FnMut(&mut RunCtx, &mut DrawContext) -> (),
    {
        let WGPUImguiContext { imgui, wgpu_ctx } = self;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = wgpu_ctx.window.inner_size();

                let surface_desc = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: PRESENT_MODE,
                };

                wgpu_ctx.surface.configure(&wgpu_ctx.device, &surface_desc);
            }
            Event::MainEventsCleared => wgpu_ctx.window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - *last_frame);
                *last_frame = now;

                let frame = match wgpu_ctx.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        logger::warning!("dropped frame: {:?}", e);
                        return;
                    }
                };

                let mut encoder: wgpu::CommandEncoder = wgpu_ctx
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu_ctx.clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                wgpu_ctx
                    .platform
                    .prepare_frame(imgui.io_mut(), &wgpu_ctx.window)
                    .expect("Failed to prepare frame");

                let ui = imgui.frame();

                /* Call user code. */
                f(
                    ctx,
                    &mut DrawContext {
                        ui: &ui,
                        wgpu_ctx: wgpu_ctx,
                    },
                );

                wgpu_ctx
                    .renderer
                    .render(ui.render(), &wgpu_ctx.queue, &wgpu_ctx.device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass);

                wgpu_ctx.queue.submit(Some(encoder.finish()));

                frame.present();
            }
            _ => {}
        }
        wgpu_ctx
            .platform
            .handle_event(self.imgui.io_mut(), &mut wgpu_ctx.window, &event);
    }
}

impl WGPUContext {
    pub fn create_texture<const WIDTH: usize, const HEIGHT: usize>(
        &mut self,
        data: [[BGRA; WIDTH]; HEIGHT],
    ) -> imgui::TextureId {
        let texture = imgui_wgpu::Texture::new(
            &mut self.device,
            &mut self.renderer,
            imgui_wgpu::TextureConfig {
                size: wgpu::Extent3d {
                    width: WIDTH as u32,
                    height: HEIGHT as u32,
                    ..Default::default()
                },
                sampler_desc: SamplerDescriptor {
                    mag_filter: FilterMode::Nearest,
                    min_filter: FilterMode::Nearest,
                    mipmap_filter: FilterMode::Nearest,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        texture.write(
            &mut self.queue,
            unsafe { to_byte_slice(&data) },
            WIDTH as u32,
            HEIGHT as u32,
        );
        self.renderer.textures.insert(texture)
    }

    pub fn update_texture(
        &mut self,
        texture_id: imgui::TextureId,
        data: &[u8],
        width: u32,
        height: u32,
    ) {
        let texture = self.renderer.textures.get_mut(texture_id).unwrap();
        texture.write(&mut self.queue, data, width, height);
    }

    pub fn destroy_texture(&mut self, _texture_id: imgui::TextureId) {}
}
