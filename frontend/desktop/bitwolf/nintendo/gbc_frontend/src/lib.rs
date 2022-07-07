mod backend;
mod config;
mod constraints;
mod messages;
mod resources;
mod state;

use anyhow::{anyhow, Result};
use common_frontend::framebuffer;
use gbc_backend::{engines::interpreter::input::InputState, Builder};
use std::{
    path::Path,
    sync::mpsc::{sync_channel, SyncSender},
};

type FrameBuffer = framebuffer::access::AccessR<gbc_backend::Texture>;
type MsgQ = util::bdq::Bdq<backend::debug::messages::CtoF, messages::FtoC>;

pub struct GBC {
    fb: FrameBuffer,
    state: state::State,
    resources: resources::Resources,
    msgq: MsgQ,
    input: SyncSender<InputState>,
    input_state: InputState,
}

impl GBC {
    pub fn new(path: &Path, wgpu_ctx: &mut imgui::WGPUContext) -> Result<Self> {
        let rom =
            std::fs::read(path).or_else(|_| Err(anyhow!("Unabel to read rom path {path:?}")))?;
        let bootrom = config::bootrom::load_bootrom()?;
        let (reader, writer) = framebuffer::buffers::triple::new::<gbc_backend::Texture>();
        let (bdq, bdq_backend) = util::bdq::new_pair(100);
        let (sender, receiver) = sync_channel(100);
        std::thread::spawn(move || {
            backend::debug::run(Builder { rom, bootrom }, bdq_backend, receiver, writer)
        });
        Ok(Self {
            fb: reader,
            state: state::State::default(),
            resources: resources::Resources::new(wgpu_ctx),
            msgq: bdq,
            input: sender,
            input_state: InputState::new(),
        })
    }
}

impl common_frontend::Frontend for GBC {}
