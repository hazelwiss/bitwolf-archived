mod backend;
mod config;
mod constraints;
mod messages;
mod state;

use anyhow::{anyhow, Result};
use common_core::framebuffer;
use gbc_backend::Builder;
use std::path::Path;

type FrameBuffer = framebuffer::access::AccessR<gbc_backend::Texture>;

pub struct GBC {
    fb: FrameBuffer,
    display_texture: imgui::gui::TextureId,
    bdq: util::bdq::Bdq<messages::CtoF, messages::FtoC>,
}

impl GBC {
    pub fn new(path: &Path, wgpu_ctx: &mut imgui::WGPUContext) -> Result<Self> {
        let rom =
            std::fs::read(path).or_else(|_| Err(anyhow!("Unabel to read rom path {path:?}")))?;
        let bootrom = config::bootrom::load_bootrom()?;
        let (reader, writer) = framebuffer::buffers::triple::new::<gbc_backend::Texture>();
        let (bdq, bdq_backend) = util::bdq::new_pair(100);
        std::thread::spawn(move || backend::run(Builder { rom, bootrom }, bdq_backend, writer));
        let display_texture = wgpu_ctx.create_texture([[util::colour::BGRA::WHITE; 160]; 144]);
        Ok(Self {
            fb: reader,
            display_texture,
            bdq,
        })
    }
}

impl common_frontend::Frontend for GBC {}
