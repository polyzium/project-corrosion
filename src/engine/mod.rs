mod pattern;
mod playlist;
mod project;
mod state;
mod utils;

use std::io::Write;
use sdl2::audio::AudioCallback;
use self::{project::Project, playlist::Playlist, state::{State, PlaylistState, PatternState}, pattern::Pattern};

#[allow(dead_code)]
pub struct DAWEngine {
	tick_length: u32,
	row_length: u32, // in ticks
	samplerate: u32,
	channels: u8,
	sample_size: u32,

	samples_passed: u32,
	song_mode: bool, // play pattern or song?
	current_pattern: usize,

	project: Project,
	state: State
}

#[allow(dead_code)]
impl DAWEngine {
	pub fn new(samplerate: u32, channels: u8, sample_size: u32) -> Self {
		let playlist = Playlist { clips: Vec::new() };
		let project = Project {
			ppq: 96,
			tempo: 125,
			rpb: 4,

			playlist,
			patterns: Vec::new(),
		};

		let mut engine = DAWEngine {
			tick_length: 0,
			row_length: 0,
			samplerate,
			channels,
			sample_size,

			samples_passed: 0,
			song_mode: false,
			current_pattern: 0,

			project,
			state: State {
				playing: true,
				playlist: PlaylistState { position: 0 },
				patterns: Vec::<PatternState>::new(),
			}
		};
		engine.set_tempo(130);
		engine.set_rpb(4);
		engine.add_pattern(Pattern::new(8, 64));

		engine.pattern_play(engine.current_pattern, 1300);

		engine
	}

	fn set_tempo(&mut self, tempo: u16) {
		self.project.tempo = tempo;
		self.tick_length = (((60.0/self.project.tempo as f32)*self.samplerate as f32)/self.project.ppq as f32) as u32;
	}

	fn set_rpb(&mut self, rpb: u8) {
		self.project.rpb = rpb;
		self.row_length = self.project.ppq as u32/self.project.rpb as u32;
	}

	fn tick(&mut self) {
		if self.state.playing {
			if self.samples_passed == 0 {
				if self.song_mode { // Playlist
					self.state.playlist.position += 1;
					print!("Ticks passed: {}\nBeats passed: {}\r\x1b[A", self.state.playlist.position, self.state.playlist.position/self.project.ppq as u32);
					std::io::stdout().flush().unwrap();
					for i in 0..self.state.patterns.len() {
						self.pattern_tick(i)
					}
				} else { // Pattern
					if !self.state.patterns[self.current_pattern].playing {
						self.pattern_play(self.current_pattern, 0);
					}
					self.pattern_tick(self.current_pattern);
				}
			}
			self.samples_passed += 1;
			if self.samples_passed >= self.tick_length {
				self.samples_passed = 0;
			}
		}
	}

	fn process(&mut self) -> &[f32] {
		self.tick();

		// TODO audio
		&[1.0, -1.0]
	}


}

/*
	AUDIO BACKENDS
*/

// SDL and CPAL
impl AudioCallback for DAWEngine {
	type Channel = f32;

	fn callback(&mut self, out: &mut [f32]) {
		for chunk in out.chunks_mut(self.channels.into()) {
			let channels = self.channels as usize;
			let sound = self.process();

			for i in 0..channels {
				chunk[i] = sound[i]
			}
		}
	}
}