use imgui::{Context, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pollster::block_on;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

const PRESENT_MODE: wgpu::PresentMode = wgpu::PresentMode::Fifo;

struct WGPUInstance {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
}

fn init_wgpu(window: &Window, imgui: &mut Context) -> WGPUInstance {
    let window_size = window.inner_size();

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

    let render_config = RendererConfig {
        texture_format: surface_desc.format,
        ..Default::default()
    };

    let renderer = Renderer::new(imgui, &device, &queue, render_config);
    WGPUInstance {
        surface,
        device,
        queue,
        renderer,
    }
}

pub struct ImguiCtx<'a> {
    ui: &'a imgui::Ui<'a>,
    wgpu_instance: &'a mut WGPUInstance,
}

impl<'a> ImguiCtx<'a> {
    pub fn ui(&self) -> &imgui::Ui<'a> {
        self.ui
    }

    pub fn _create_texture<const WIDTH: usize, const HEIGHT: usize>(
        &mut self,
        //data: [[_; WIDTH]; HEIGHT],
    ) /*-> imgui::TextureId*/
    {
        //let texture = imgui_wgpu::Texture::new(
        //    &mut self.device,
        //    &mut self.renderer,
        //    imgui_wgpu::TextureConfig {
        //        size: wgpu::Extent3d {
        //            width: WIDTH as u32,
        //            height: HEIGHT as u32,
        //            ..Default::default()
        //        },
        //        sampler_desc: SamplerDescriptor {
        //            mag_filter: FilterMode::Nearest,
        //            min_filter: FilterMode::Nearest,
        //            mipmap_filter: FilterMode::Nearest,
        //            ..Default::default()
        //        },
        //        ..Default::default()
        //    },
        //);
        //texture.write(
        //    &mut self.queue,
        //    unsafe { to_byte_slice(&data) },
        //    WIDTH as u32,
        //    HEIGHT as u32,
        //);
        //self.renderer.textures.insert(texture)
    }

    pub fn _update_texture(
        &mut self,
        //texture_id: imgui::TextureId,
        //data: &[u8],
        //width: u32,
        //height: u32,
    ) {
        //let texture = self.renderer.textures.get_mut(texture_id).unwrap();
        //texture.write(&mut self.queue, data, width, height);
    }

    pub fn _destroy_texture(&mut self, _texture_id: imgui::TextureId) {}
}

pub fn run<F>(mut draw_pass: F)
where
    F: FnMut(ImguiCtx) + 'static,
{
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Unable to create window using winit");

    let mut imgui = Context::create();

    let hidpi_factor = window.scale_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let mut wgpu_instance = init_wgpu(&window, &mut imgui);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

    let clear_colour = wgpu::Color {
        r: 0.0,
        g: 0.2,
        b: 0.4,
        a: 1.0,
    };

    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = wgpu_instance
                    .surface
                    .get_current_texture()
                    .expect("Dropped frame");

                let mut encoder = wgpu_instance
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
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
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                draw_pass(ImguiCtx {
                    ui: &ui,
                    wgpu_instance: &mut wgpu_instance,
                });

                platform.prepare_render(&ui, &window);
                wgpu_instance
                    .renderer
                    .render(
                        ui.render(),
                        &wgpu_instance.queue,
                        &wgpu_instance.device,
                        &mut rpass,
                    )
                    .expect("Render failed!");

                drop(rpass);

                wgpu_instance.queue.submit(Some(encoder.finish()));

                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = window.inner_size();
                if size.width > 0 && size.height > 0 {
                    let surface_desc = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width as u32,
                        height: size.height as u32,
                        present_mode: PRESENT_MODE,
                    };
                    wgpu_instance
                        .surface
                        .configure(&wgpu_instance.device, &surface_desc);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
        platform.handle_event(imgui.io_mut(), &window, &event)
    });
}
