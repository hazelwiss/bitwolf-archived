use std::path::PathBuf;

use imgui::{Context, FontConfig, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use super::window::WindowGfx;

pub struct ImguiCtx {
    pub ctx: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl ImguiCtx {
    pub fn new(window: &mut WindowGfx, ini_file: Option<PathBuf>) -> Self {
        let mut ctx = Context::create();
        ctx.set_ini_filename(ini_file);
        let mut platform = WinitPlatform::init(&mut ctx);
        platform.attach_window(ctx.io_mut(), &window.window, HiDpiMode::Default);
        let hpdi_factor = window.scale_factor();
        ctx.io_mut().font_global_scale = (1.0 / hpdi_factor) as f32;
        let font_size = (13.0 * hpdi_factor) as f32;
        ctx.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let gfx = &mut window.gfx;
        let renderer = Renderer::new(
            &mut ctx,
            &mut gfx.device,
            &mut gfx.queue,
            RendererConfig {
                texture_format: gfx.texture_format,
                ..Default::default()
            },
        );
        Self {
            ctx,
            platform,
            renderer,
        }
    }
}
