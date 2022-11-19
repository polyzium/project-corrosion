mod pattern;
mod playlist;

use std::io::Write;

use sdl2::audio::AudioCallback;

#[allow(dead_code)]
pub struct DAWEngine {
	tick_length: u32,
	samplerate: u32,
	channels: u8,
	sample_size: u32,

	ppq: u16, // pulses per quarter
	tempo: u16,
	pub rpb: u8, // rows per beat

	samples_passed: u32,
	ticks_passed: u16
}

#[allow(dead_code)]
impl DAWEngine {
	pub fn new(samplerate: u32) -> Self {
		let mut engine = DAWEngine {
			tick_length: 0,
			samplerate,
			channels: 2,
			sample_size: 512,
		
			ppq: 96,
			tempo: 125,
			rpb: 4,

			samples_passed: 0,
			ticks_passed: 0
		};
		engine.set_tempo(120);

		engine
	}

	fn set_tempo(&mut self, tempo: u16) {
		self.tempo = tempo;
		self.tick_length = (((60.0/self.tempo as f32)*self.samplerate as f32)/self.ppq as f32) as u32;
	}

	fn process(&mut self) -> &[f32] {
		//TODO actual logic
		if self.samples_passed >= self.tick_length {
			self.ticks_passed += 1;
			print!("Ticks passed: {}\nBeats passed: {}\r\x1b[A", self.ticks_passed, self.ticks_passed/self.ppq);
			std::io::stdout().flush().unwrap();
			self.samples_passed = 0;
		} else {
			self.samples_passed += 1;
		}

		&[0.0, 0.0]
	}
}

/*
	AUDIO BACKENDS
*/

// SDL
impl AudioCallback for DAWEngine {
	type Channel = f32;

	fn callback(&mut self, out: &mut [f32]) {
		for chunk in out.chunks_mut(self.channels.into()) {
			let sound = self.process();

			// TODO adapt for multichannel
			chunk[0] = sound[0];
			chunk[1] = sound[1];
		}
	}
}