use cpal::{
    traits::{DeviceTrait, HostTrait},
    Sample, StreamConfig,
};

pub fn test() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_config_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_config_range
        .next()
        .expect("No supported config.")
        .with_max_sample_rate();
    let config = supported_config.config();
    let mut x = 0.0f32;
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = Sample::from(&(x.sin() * 2.0));
                    x += 0.01;
                }
            },
            |err| println!("{err:?}"),
        )
        .expect("failed to create stream");
    use cpal::traits::StreamTrait;
    stream.play().expect("unable to play stream");
    loop {}
}
