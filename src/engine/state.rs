use std::io::Write;

use super::{DAWEngine, project::Project};

pub struct State {
    pub playing: bool,
    pub playlist: PlaylistState,
    pub patterns: Vec<PatternState>
}

pub struct PlaylistState {
	pub position: u32
}

impl PlaylistState {
	fn advance(&mut self) {
		self.position += 1
	}

	fn seek(&mut self, pos: u32) {
		// TODO stop/choke plugin voices
		self.position = pos
	}
}

#[derive(Debug)]
pub struct PatternState {
	pub pattern_index: usize, // I hate indices so much, it's the worst way to point to data, please fix
    pub ticks_passed: u16,
	pub position: u32, // in ticks
	pub playing: bool,
	pub row: u16
}

impl DAWEngine {
    pub fn pattern_play(&mut self, index: usize, offset: u32) {
        let state = &mut self.state.patterns[index];

        state.ticks_passed = 0;
        state.position = offset;
        state.row = (state.position/(self.project.ppq as u32/self.project.rpb as u32)) as u16;
        state.ticks_passed = (offset % self.row_length) as u16;
        state.playing = true;
    }
    
    pub fn pattern_stop(&mut self, index: usize) {
        self.state.patterns[index].playing = false
        // TODO note off to all active voices
    }

    pub fn pattern_tick(&mut self, index: usize) {
        let state = &mut self.state.patterns[index];
        if !state.playing { return }

        let pat = &self.project.patterns[state.pattern_index];
    
        if state.playing {
            if state.ticks_passed as u32 == self.row_length {
                state.row += 1;
                state.ticks_passed = 0;
            }
        }

        if state.ticks_passed == 0 {
            if state.row as usize != pat.rows.len() {
                self.pattern_play_row(index);
            }
        }

        let state = &mut self.state.patterns[index];

        state.position += 1;
        state.ticks_passed += 1;

        if state.position as usize >= pat.rows.len()*self.row_length as usize {
            state.playing = false;
            state.position = 0;
            state.row = 0;
            state.ticks_passed = 0;
        }
    }

    fn pattern_play_row(&self, index: usize) {
        let state = &self.state.patterns[index];

        let pat = &self.project.patterns[state.pattern_index];
        let row = &pat.rows[state.row as usize];
    
        // TODO note playback
        // enjoy debug information for now
        print!("{:0>2} | ", state.row);
        for track in row.iter() {
            print!("{} {:0>2} {} | ",
                format_note(track.note),
                if track.instrument == 0 { "..".to_string() } else {
                    track.instrument.to_string()
                },
                if track.volume > 127 { " ...".to_string() } else {
                    track.volume.to_string()
                },
            );
        }
        print!("\n")
    }
}

fn format_note(note: u8) -> String {
    match note {
        255 => return "...".to_string(), // none
        128 => return "Off".to_string(), // off
        129 => return "Cut".to_string(), // cut/choke
        130 => return "Fde".to_string(), // fade
        _ => {}
    }
    let mut out = String::new();
    out.push_str(match note % 12 {
        0 => "C-",
        1 => "C#",
        2 => "D-",
        3 => "D#",
        4 => "E-",
        5 => "F-",
        6 => "F#",
        7 => "G-",
        8 => "G#",
        9 => "A-",
        10 => "A#",
        11 => "B-",
        _ => unreachable!()
    });
    out.push_str(format!("{}", note/12).as_str());
    out
}