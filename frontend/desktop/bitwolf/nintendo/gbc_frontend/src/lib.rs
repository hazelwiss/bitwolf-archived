mod backend;
mod config;
mod frontend;

use anyhow::{anyhow, Result};
use common_core::framebuffer;
use gbc_backend::Builder;
use std::path::Path;

type FrameBuffer = framebuffer::AccessR<gbc_backend::Texture>;

pub struct GBC {
    fb: FrameBuffer,
    display_texture: imgui::gui::TextureId,
}

impl GBC {
    pub fn new(path: &Path, wgpu_ctx: &mut imgui::WGPUContext) -> Result<Self> {
        let rom =
            std::fs::read(path).or_else(|_| Err(anyhow!("Unabel to read rom path {path:?}")))?;
        let bootrom = config::bootrom::load_bootrom()?;
        let (reader, writer) = framebuffer::fb_3b::<gbc_backend::Texture>();
        std::thread::spawn(move || {
            backend::run(Builder {
                rom,
                bootrom,
                fb: writer,
            })
        });
        let display_texture = wgpu_ctx.create_texture([[util::colour::BGRA::WHITE; 160]; 144]);
        Ok(Self {
            fb: reader,
            display_texture,
        })
    }
}

impl common_frontend::Frontend for GBC {}
