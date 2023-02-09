mod nds;

use crate::cla::CLA;
use crate::config::Config;
use crate::debug::{DebugCoreInterface, DebugInterface};
use crate::{core, debug};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

macro_rules! def_backend {
    ($($backend:ident : $ty:ty),*) => {
        #[derive(Clone, Copy)]
        pub enum Backend {
            $(
                $backend
            ),*
        }

        enum Core {
            $(
                $backend()
            ),*
        }
    };
}

def_backend!(NDS: ::nds::NDSInterp);

pub struct CoreBuilder {
    backend: Backend,
    path: PathBuf,
}

impl CoreBuilder {
    pub fn new(backend: Backend, path: PathBuf) -> Self {
        Self { backend, path }
    }

    pub fn from_cla(cla: &CLA) -> Option<Self> {
        if let Some(load_rom) = &cla.load_rom {
            Some(Self::new(load_rom.backend, load_rom.rom.to_path_buf()))
        } else {
            None
        }
    }

    pub fn from_config(self, config: &Config) -> Self {
        self
    }

    pub fn build_threaded<'a>(self) -> Runner {
        let shared_data = Arc::new(SharedData {
            running: AtomicBool::new(true),
            paused: AtomicBool::new(true),
        });
        let shared_data_core = shared_data.clone();
        let (dbg, dbg_core) = debug::new();
        let (core, f) = match self.backend {
            Backend::NDS => (todo!(), || {
                nds::run_core_interp_threaded(::nds::Core::new(), shared_data_core, dbg_core)
            }),
        };
        let jhandle = thread::Builder::new()
            .name("core".to_string())
            .spawn(|| match self.backend {
                Backend::NDS => {
                    nds::run_core_interp_threaded(::nds::Core::new(), shared_data_core, dbg_core)
                }
            })
            .expect("failed to create core thread");
        Runner {
            jhandle,
            shared_data,
            core: todo!(),
        }
    }

    pub fn build<'a>(self) -> () {
        unimplemented!()
    }
}

struct SharedData {
    running: AtomicBool,
    paused: AtomicBool,
}

trait InterfaceImpl {
    type CtoF;
    type FtoC;
    type CData: Default;
    type FData: Default;
}

struct CoreInterface<I: InterfaceImpl> {
    shared: Arc<SharedData>,
    recv: crossbeam::Receiver<I::FtoC>,
    send: crossbeam::Sender<I::CtoF>,
    data: I::CData,
}

impl<I: InterfaceImpl> CoreInterface<I> {
    pub fn pause(&self) {}
}

struct Interface<I: InterfaceImpl> {
    shared: Arc<SharedData>,
    recv: crossbeam::Receiver<I::CtoF>,
    send: crossbeam::Sender<I::FtoC>,
    data: I::FData,
}

impl<I: InterfaceImpl> Interface<I> {
    pub fn close(self) {}

    pub fn pause(&self) {}
}

fn new_interface<I: InterfaceImpl>() -> (Interface<I>, CoreInterface<I>) {
    let cap = 100;
    let (s0, r0) = crossbeam::bounded(cap);
    let (s1, r1) = crossbeam::bounded(cap);
    let shared_data = Arc::new(SharedData {
        running: AtomicBool::new(true),
        paused: AtomicBool::new(true),
    });
    (
        Interface {
            shared: shared_data.clone(),
            recv: r0,
            send: s1,
            data: Default::default(),
        },
        CoreInterface {
            shared: shared_data,
            recv: r1,
            send: s0,
            data: Default::default(),
        },
    )
}

pub struct Runner {
    jhandle: JoinHandle<()>,
    shared_data: Arc<SharedData>,
    core: Core,
}

impl Runner {
    fn close(self) {
        self.shared_data.running.store(false, Ordering::Relaxed);
        self.jhandle.join();
    }

    fn update(&mut self) {}
}
