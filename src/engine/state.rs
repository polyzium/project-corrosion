use std::io::Write;

use super::{pattern::Note, project::Project, DAWEngine};

pub struct State {
    pub playing: bool,
    pub playlist: PlaylistState,
    pub patterns: Vec<PatternState>,
}

pub struct PlaylistState {
    pub position: u32,
}

impl PlaylistState {
    fn advance(&mut self) {
        self.position += 1
    }

    fn seek(&mut self, pos: u32) {
        // TODO stop/choke plugin voices for DAWEngine
        self.position = pos
    }
}

#[derive(Debug)]
pub struct PatternState {
    pub pattern_index: usize,
    pub(crate) ticks_passed: u16,
    // TODO set_rpb method for DAWEngine mutating Pattern's rpb and PatternState's row_length.
    // 2022-04-07: row_length has been moved from DAWEngine to here. See the comment in the Pattern struct (same reason)
    pub(crate) row_length: u32, // in ticks
    pub position: u32,          // in ticks
    pub playing: bool,
    pub row: u16,
}

impl DAWEngine {
    pub fn pattern_play(&mut self, index: usize, offset: u32) {
        let state = &mut self.state.patterns[index];

        state.ticks_passed = 0;
        state.position = offset;
        state.row = (state.position
            / (self.project.ppq as u32 / self.project.patterns[index].rpb as u32))
            as u16;
        state.ticks_passed = (offset % state.row_length) as u16;
        state.playing = true;
    }

    pub fn pattern_stop(&mut self, index: usize) {
        self.state.patterns[index].playing = false
        // TODO note off to all active voices
    }

    pub fn pattern_tick(&mut self, index: usize) {
        let state = &mut self.state.patterns[index];
        if !state.playing {
            return;
        }

        let pat = &self.project.patterns[state.pattern_index];

        if state.ticks_passed as u32 == state.row_length {
            state.row += 1;
            state.ticks_passed = 0;
        }

        if state.ticks_passed == 0 {
            if state.row as usize != pat.rows.len() {
                self.pattern_play_row(index);
            }
        }

        let state = &mut self.state.patterns[index];

        state.position += 1;
        state.ticks_passed += 1;

        if state.position as usize >= pat.rows.len() * state.row_length as usize {
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
            print!(
                "{} {:0>2} {} | ",
                format_note(track.note),
                if track.instrument == 0 {
                    "..".to_string()
                } else {
                    track.instrument.to_string()
                },
                if track.volume > 127 {
                    " ...".to_string()
                } else {
                    track.volume.to_string()
                },
            );
        }
        print!("\n")
    }
}

fn format_note(note: Note) -> String {
    match note {
        Note::None => return "...".to_string(),            // none
        Note::PreviousChannel => return "<<<".to_string(), // prev channel
        Note::Off => return "Off".to_string(),             // off
        Note::Cut => return "Cut".to_string(),             // cut/choke
        Note::Fade => return "Fde".to_string(),            // fade
        _ => {}
    }
    let mut out = String::new();
    out.push_str(match note as u8 % 12 {
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
        _ => unreachable!(),
    });
    out.push_str(format!("{}", note as u8 / 12).as_str());
    out
}
