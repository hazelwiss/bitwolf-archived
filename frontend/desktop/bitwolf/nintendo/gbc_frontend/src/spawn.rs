use self::backend::Com;

use super::debug;
use anyhow::{anyhow, Result};
use common_frontend::framebuffer::{self, access};
use gbc_backend::{input::InputState, Builder};
use std::{
    path::Path,
    sync::{
        atomic::AtomicBool,
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};
use util::bdq::Bdq;

type Texture = gbc_backend::Texture;

pub mod frontend {
    use super::*;

    pub struct Com {
        pub input_state: InputState,
        pub input_sender: SyncSender<InputState>,
        pub fb_reader: access::AccessR<Texture>,
        pub msgq: Bdq<debug::messages::CtoF, debug::messages::FtoC>,
        pub state: debug::state::State,
        pub running: Arc<AtomicBool>,
    }
}

pub mod backend {
    use super::*;

    pub struct Com {
        pub input_state: InputState,
        pub input_recv: Receiver<InputState>,
        pub fb_writer: access::AccessW<Texture>,
        pub msgq: Bdq<debug::messages::FtoC, debug::messages::CtoF>,
        pub running: Arc<AtomicBool>,
    }
}

pub fn spawn<F>(path: &Path, mut f: F) -> Result<frontend::Com>
where
    F: FnMut(Builder, Box<Com>) + Send + 'static,
{
    const CHANNEL_SIZE: usize = 100;
    let (fb_reader, fb_writer) = framebuffer::buffers::triple::new::<Texture>();
    let (frontend, core) = util::bdq::new_pair(CHANNEL_SIZE);
    let (input_sender, input_recv) = sync_channel(CHANNEL_SIZE);
    let running = Arc::new(AtomicBool::new(true));
    let mut com = Box::new(backend::Com {
        input_state: InputState::new(),
        input_recv,
        fb_writer,
        msgq: core,
        running: running.clone(),
    });
    let com_ptr = com.as_mut() as *mut _;
    let builder = Builder {
        rom: std::fs::read(path).or_else(|_| Err(anyhow!("Unabel to read rom path {path:?}")))?,
        bootrom: super::config::bootrom::load_bootrom()?,
        audio_interface: Box::new(super::audio::Audio::new()),
        video_interface: Box::new(super::video::Video::new(com_ptr)),
        input_interface: Box::new(super::input::Input::new(com_ptr)),
    };
    std::thread::spawn(move || f(builder, com));
    Ok(frontend::Com {
        input_state: InputState::new(),
        input_sender,
        fb_reader,
        msgq: frontend,
        state: debug::state::State::default(),
        running: running.clone(),
    })
}
