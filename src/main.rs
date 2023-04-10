use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize::Fixed;

mod engine;

fn main() {
    let samplerate = 48000;
    let channels = 2u8;
    let mut daw = engine::DAWEngine::new(samplerate, 2, 512);

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output devices available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("Unable to query configs");
    // let supported_config = supported_configs_range.next().expect("No supported config found").with_max_sample_rate().into();

    let mut config: cpal::StreamConfig = supported_configs_range
        .find(|conf| conf.channels() == channels.into())
        .expect(&format!("Unable to find a {}-channel config", channels))
        .with_sample_rate(cpal::SampleRate(samplerate))
        .into();

    config.buffer_size = Fixed(512);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| daw.callback(data),
            move |err| eprintln!("Audio error: {}", err),
        )
        .expect("Cannot set up output stream");
    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(4))
}
