mod backend;
mod config;
mod constraints;
mod messages;
mod resources;
mod state;

use anyhow::{anyhow, Result};
use common_frontend::framebuffer;
use gbc_backend::Builder;
use std::path::Path;

type FrameBuffer = framebuffer::access::AccessR<gbc_backend::Texture>;

pub struct GBC {
    fb: FrameBuffer,
    state: state::State,
    resources: resources::Resources,
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
        Ok(Self {
            fb: reader,
            state: state::State::default(),
            resources: resources::Resources::new(wgpu_ctx),
            bdq,
        })
    }
}

impl common_frontend::Frontend for GBC {}
