use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, Device, Sample, SampleRate, StreamConfig,
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
        let mut config = supported_config.config();
        config.buffer_size = BufferSize::Default;
        logger::info!("Created audio loopback: {config:?}");
        Self { device, config }
    }

    pub fn play<T: Copy + Sample + Default + Send + Sync + 'static, const SIZE: usize>(
        self,
    ) -> AudioContext<T, SIZE> {
        use cpal::traits::StreamTrait;
        let (pusher, poper) = ring_buffer::spawn::<T, SIZE>();
        let barrier = Arc::new(Barrier::new(2));
        let barrier_thread = barrier.clone();
        std::thread::spawn(move || {
            let stream = self
                .device
                .build_output_stream(
                    &self.config,
                    move |data: &mut [T], _| {
                        for sample in data.iter_mut() {
                            *sample = poper.pop().unwrap_or_default();
                        }
                    },
                    |err| logger::warning!("[AUDIO] error: {err:?}"),
                )
                .expect("[AUDIO] failed to create stream");
            stream.play().expect("[AUDIO] unable to play stream");
            barrier_thread.wait();
        });

        AudioContext {
            rb: pusher,
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
