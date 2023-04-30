pub mod pattern;
pub mod playlist;
pub mod project;
pub mod state;
pub mod utils;
pub mod plugins;
mod test;

use self::{
    playlist::Playlist,
    project::Project,
    state::{PatternState, PlaylistState, State}, test::GoertzelSine,
    plugins::interface::NoteState
};
use std::io::Write;

#[allow(dead_code)]
pub struct DAWEngine {
    tick_length: u32,
    samplerate: u32,
    channels: u8,
    sample_size: u32,

    samples_passed: u32,
    song_mode: bool, // play pattern or song?
    current_pattern: usize,

    pub project: Project,
    pub state: State,

    test_osc: GoertzelSine
}

// These two functions below should belong to DAWEngine, but because of borrow checking issues, they're separate.
// Please point note_registry to &self.state.notes, where self = DAWEngine.

/*
    Allocate a note with the desired slot ID. If it's occupied, select any free slot.
    Returns slot ID.
*/
fn allocate_note(note_registry: &mut [NoteState], note: u8, instrument: usize, mut id: usize) -> usize {
    if note_registry[id].is_on {
        let mut found = false;

        // Find a free slot
        for (index, state) in note_registry.iter().enumerate() {
            if state.is_on {
                continue
            } else {
                id = index;
                found = true;
                break;
            }
        }

        // If all the slots are occupied
        if !found {
            panic!("Too many playing notes!")
        }
    }

    note_registry[id].key = note;
    note_registry[id].instrument = instrument;
    note_registry[id].id = id;

    return note_registry[id].id
}

fn free_note(note_registry: &mut [NoteState], id: usize) {
    note_registry[id].is_on = false;
}

#[allow(dead_code)]
impl DAWEngine {
    pub fn new(samplerate: u32, channels: u8, sample_size: u32) -> Self {
        let playlist = Playlist { clips: Vec::new() };
        let project = Project {
            ppq: 96,
            tempo: 125,

            playlist,
            patterns: Vec::new(),
        };

        let mut engine = DAWEngine {
            tick_length: 0,
            samplerate,
            channels,
            sample_size,

            samples_passed: 0,
            song_mode: false,
            current_pattern: 0,

            project,
            state: State {
                playing: false,
                playlist: PlaylistState { position: 0 },
                patterns: Vec::<PatternState>::new(),
                event_list: Vec::with_capacity(sample_size as usize),
                notes: {
                    let mut notes: Vec<NoteState> = Vec::with_capacity(256);
                    notes.resize_with(256, || NoteState { id: 0, instrument: 0, key: 0, vel: 0, pitch_bend: 0.0, is_on: false });
                    notes
                },
                next_note_id: 0,
            },

            test_osc: GoertzelSine::new(440.0, 0.0, 48000)
        };
        engine.set_tempo(125);
        // engine.add_pattern(Pattern::new(8, 64));
        // engine.add_pattern(test_pattern!());

        // engine.pattern_play(engine.current_pattern, 1300);
        // engine.pattern_play(engine.current_pattern, 48);

        engine
    }

    pub fn set_tempo(&mut self, tempo: u16) {
        self.project.tempo = tempo;
        self.tick_length = (((60.0 / self.project.tempo as f32) * self.samplerate as f32)
            / self.project.ppq as f32) as u32;
    }

    fn tick(&mut self, sample_index: usize) {
        if !self.state.playing {
            return;
        };
        if self.samples_passed == 0 {
            if self.song_mode {
                // Playlist
                self.state.playlist.position += 1;
                print!(
                    "Ticks passed: {}\nBeats passed: {}\r\x1b[A",
                    self.state.playlist.position,
                    self.state.playlist.position / self.project.ppq as u32
                );
                std::io::stdout().flush().unwrap();
                for i in 0..self.state.patterns.len() {
                    self.pattern_tick(i, sample_index)
                }
            } else {
                // Pattern
                if !self.state.patterns[self.current_pattern].playing {
                    self.pattern_play(self.current_pattern, self.state.patterns[self.current_pattern].position);
                }
                self.pattern_tick(self.current_pattern, sample_index);
            }
        }
        self.samples_passed += 1;
        if self.samples_passed >= self.tick_length {
            self.samples_passed = 0;
        }
    }

    pub fn switch_song_mode(&mut self, song_mode: bool) {
        // Stop all clips

        // Patterns
        for i in 0..self.state.patterns.len() {
            self.pattern_stop(i)
        }
        // Note to self: whenever a new clip type is added, stop it here
        self.state.playlist.position = 0;

        self.song_mode = song_mode;
    }

    pub fn process(&mut self, buf: &mut [f32]) {
        for sample_index in 0..(buf.len()/self.channels as usize) {
            self.tick(sample_index);
        }

        // TODO audio
        /* for chunk in buf.chunks_mut(self.channels as usize) {
            (chunk[0], chunk[1]) = (1.0, -1.0)
        } */

        // play test tone
        /* for chunk in buf.chunks_mut(self.channels as usize) {
            let smp = self.test_osc.process();
            for channel in chunk {
                *channel = smp
            }
        } */
    }
}

/*
    AUDIO BACKENDS
*/

// CPAL
impl DAWEngine {
    pub fn callback(&mut self, out: &mut [f32]) {
        self.process(out)
    }
}