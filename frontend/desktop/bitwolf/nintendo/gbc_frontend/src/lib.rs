mod audio;
mod backend;
mod config;
mod constraints;
mod messages;
mod resources;
mod state;

use anyhow::{anyhow, Result};
use audio::AudioSampler;
use common_frontend::{framebuffer, FrontendWrapper};
use gbc_backend::{engines::interpreter::input::InputState, Builder};
use std::{
    path::Path,
    sync::{
        atomic::AtomicBool,
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
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
    running: Arc<AtomicBool>,
}

impl GBC {
    pub fn new(path: &Path, wgpu_ctx: &mut imgui::WGPUContext) -> Result<FrontendWrapper> {
        let rom =
            std::fs::read(path).or_else(|_| Err(anyhow!("Unabel to read rom path {path:?}")))?;
        let bootrom = config::bootrom::load_bootrom()?;
        let (reader, writer) = framebuffer::buffers::triple::new::<gbc_backend::Texture>();
        let (bdq, bdq_backend) = util::bdq::new_pair(100);
        let (sender, receiver) = sync_channel(100);
        let audio = AudioSampler::new();
        let running = Arc::new(AtomicBool::new(true));
        let running_thread = running.clone();
        std::thread::spawn(move || {
            backend::debug::run(
                Builder { rom, bootrom },
                running_thread,
                bdq_backend,
                receiver,
                writer,
                audio,
            )
        });
        Ok(FrontendWrapper::new(Box::new(Self {
            fb: reader,
            state: state::State::default(),
            resources: resources::Resources::new(wgpu_ctx),
            msgq: bdq,
            input: sender,
            input_state: InputState::new(),
            running,
        })))
    }
}

impl common_frontend::Frontend for GBC {}
