use std::time::Duration;

mod engine;

fn main() {
	let daw = engine::DAWEngine::new(48000);

	let sdl = sdl2::init().unwrap();
	let audio = sdl.audio().unwrap();
	let spec = sdl2::audio::AudioSpecDesired{ freq: Some(48000), channels: Some(2), samples: Some(512) };

	let device = audio.open_playback(None, &spec, |_| { daw }).unwrap();
	device.resume();

	std::thread::sleep(Duration::from_secs(3))
}
