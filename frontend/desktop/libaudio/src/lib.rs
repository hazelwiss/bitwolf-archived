use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Sample, StreamConfig,
};
use std::sync::{Arc, Barrier};
use util::ring_buffer;

pub struct AudioBuilder {
    device: Device,
    config: StreamConfig,
}

impl AudioBuilder {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("[AUDIO] no output device available");
        let mut supported_config_range = device
            .supported_output_configs()
            .expect("[AUDIO] error while querying configs");
        let supported_config = supported_config_range
            .next()
            .expect("[AUDIO] No supported config.")
            .with_max_sample_rate();
        let config = supported_config.config();
        Self { device, config }
    }

    pub fn play<T: Copy + Sample + Default + Send + 'static, const SIZE: usize>(
        self,
    ) -> AudioContext<T, SIZE> {
        let rb = ring_buffer::MPRB::<T, SIZE>::new();
        let poper = rb.poper();
        let barrier = Arc::new(Barrier::new(2));
        let barrier_thread = barrier.clone();
        std::thread::spawn(move || {
            let stream = self
                .device
                .build_output_stream(
                    &self.config,
                    move |data: &mut [T], _| {
                        for sample in data.iter_mut() {
                            *sample = if let Some(new_sample) = poper.pop() {
                                new_sample
                            } else {
                                T::default()
                            };
                        }
                    },
                    |err| logger::warning!("[AUDIO] error: {err:?}"),
                )
                .expect("[AUDIO] failed to create stream");
            use cpal::traits::StreamTrait;
            stream.play().expect("[AUDIO] unable to play stream");
            barrier_thread.wait();
        });

        AudioContext {
            rb: rb.pusher(),
            barrier,
        }
    }
}

pub struct AudioContext<T: Copy, const SIZE: usize> {
    rb: ring_buffer::RBPusher<T, SIZE>,
    barrier: Arc<Barrier>,
}

impl<T: Copy, const SIZE: usize> AudioContext<T, SIZE> {
    #[inline(always)]
    pub fn push_sample(&self, sample: T) {
        self.rb.push(sample);
    }
}

impl<T: Copy, const SIZE: usize> Drop for AudioContext<T, SIZE> {
    fn drop(&mut self) {
        self.barrier.wait();
    }
}
