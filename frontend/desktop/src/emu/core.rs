use crate::config::EmuConfig;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use util::log::{info, Logger};

pub struct CoreRunner {
    log: Logger,
    j_handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl Drop for CoreRunner {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(join) = self.j_handle.take() {
            let mut joined = false;
            let attempts = 25;
            for attempt in 0..attempts {
                if join.is_finished() {
                    join.join().expect("Failed to join joinhandle");
                    joined = true;
                    break;
                } else {
                    info!(
                        self.log,
                        "attempting to shut down core; attempt {attempt} out of {attempts}"
                    );
                    thread::sleep(Duration::from_millis(200))
                }
            }
            if !joined {
                panic!("failed to join core thread after {attempts} attempts.");
            }
        } else {
            panic!("missing join handle!")
        }
    }
}

impl CoreRunner {
    pub fn spawn(log: Logger, config: &EmuConfig, rom: Vec<u8>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let core = CoreState {
            running: running.clone(),
            rom,
        };
        let j_handle = Some(thread::spawn(move || core_main(core)));
        Self {
            j_handle,
            running,
            log,
        }
    }
}

struct CoreState {
    running: Arc<AtomicBool>,
    rom: Vec<u8>,
}

fn core_main(core: CoreState) {
    let CoreState { running, rom } = core;

    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(2500));
        println!("running core!");
    }
}
