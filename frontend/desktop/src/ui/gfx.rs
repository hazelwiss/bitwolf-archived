use imgui::{Context, DrawData, FontSource, Ui};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pollster::block_on;
use std::time::Instant;
use wgpu::{RenderPass, SurfaceConfiguration, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub(super) const PRESENT_MODE: wgpu::PresentMode = wgpu::PresentMode::Fifo;
const SURFACE_CONF: SurfaceConfiguration = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: wgpu::TextureFormat::Bgra8UnormSrgb,
    width: 0,
    height: 0,
    present_mode: PRESENT_MODE,
};

pub struct GfxContext {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
}

pub struct Builder {}

impl Builder {
    pub fn build(self, window: &Window) -> (GfxContext, imgui::Context) {
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

        let surface_desc = SurfaceConfiguration {
            width: window_size.width,
            height: window_size.height,
            ..SURFACE_CONF
        };
        surface.configure(&device, &surface_desc);

        let render_config = RendererConfig {
            texture_format: surface_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &device, &queue, render_config);
        (
            GfxContext {
                surface,
                device,
                queue,
                renderer,
            },
            imgui,
        )
    }
}

impl GfxContext {
    pub(super) fn render_pass(
        &mut self,
        clear_colour: wgpu::Color,
        draw_data: &'_ DrawData,
    ) -> SurfaceTexture {
        let frame = self.surface.get_current_texture().expect("Dropped frame");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        self.renderer
            .render(draw_data, &self.queue, &self.device, &mut pass)
            .expect("Render failed!");
        drop(pass);
        self.queue.submit(Some(encoder.finish()));
        frame
    }

    pub(super) fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            let surface_desc = wgpu::SurfaceConfiguration {
                width: size.width as u32,
                height: size.height as u32,
                ..SURFACE_CONF
            };
            self.surface.configure(&self.device, &surface_desc);
        }
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
