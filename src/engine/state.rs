use std::io::Write;
use super::{pattern::Note, project::Project, DAWEngine, plugins::interface::{TimedEvent, Event, NoteState}, allocate_note, free_note};

pub struct State {
    pub playing: bool,
    pub playlist: PlaylistState,
    pub patterns: Vec<PatternState>,

    // note: always initialize this field using Vec::with_capacity!
    pub event_list: Vec<TimedEvent>,
    pub notes: Vec<NoteState>,
    pub next_note_id: usize
}

pub struct PlaylistState {
    pub position: u32,
}

impl PlaylistState {
    fn seek(&mut self, pos: u32) {
        // TODO stop/choke plugin voices for DAWEngine
        self.position = pos
    }
}

/* #[derive(Debug)]
pub struct PatternNoteState {
    track: usize,
    instrument: usize,

    note: Note,
    pitch: f32, // in semitones, relative to current note
    velocity: u8, // 0..=127
} */

#[derive(Clone, Debug)]
pub struct PatternState {
    // pub pattern_index: usize,
    pub(crate) ticks_passed: u16,
    // TODO set_rpb method for DAWEngine mutating Pattern's rpb and PatternState's row_length.
    // 2022-04-07: row_length has been moved from DAWEngine to here. See the comment in the Pattern struct (same reason)
    pub(crate) row_length: u32, // in ticks
    pub(crate) note_ids: Vec<usize>,
    pub(crate) last_instrument: u8,

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

    pub fn pattern_tick(&mut self, index: usize, sample_index: usize) {
        let state = &mut self.state.patterns[index];
        if !state.playing {
            return;
        }

        if state.ticks_passed as u32 == state.row_length {
            state.row += 1;
            state.ticks_passed = 0;
        }

        if state.ticks_passed == 0 {
            if state.row as usize != self.project.patterns[index].rows.len() {
                self.pattern_play_row(index, sample_index);
            }
        }

        let state = &mut self.state.patterns[index];

        state.position += 1;
        state.ticks_passed += 1;

        if state.position as usize >= self.project.patterns[index].rows.len() * state.row_length as usize {
            state.playing = false;
            state.position = 0;
            state.row = 0;
            state.ticks_passed = 0;
        }
    }

    fn pattern_play_row(&mut self, index: usize, sample_index: usize) {
        let pat = &self.project.patterns[index];

        #[cfg(debug_assertions)]
        {
            print!("{:0>2} | ", self.state.patterns[index].row);
            for track in &pat.rows[self.state.patterns[index].row as usize] {
                print!(
                    "{} {:0>2} {} | ",
                    format_note(track.note),
                    if track.instrument == 0 {
                        "..".to_string()
                    } else {
                        track.instrument.to_string()
                    },
                    if track.volume > 127 {
                        "...".to_string()
                    } else {
                        format!("{:0>3}", track.volume.to_string())
                    },
                );
            }
            print!("\n");
        }

        for (track, event) in pat.rows[self.state.patterns[index].row as usize].iter().enumerate() {
            match event.note {
                Note::PreviousTrack => todo!(),
                Note::Off => {
                    let note_state = self.state.notes[self.state.patterns[index].note_ids[track]];

                    self.state.event_list.push(TimedEvent {
                        instrument: note_state.instrument,
                        position: sample_index as u32,
                        event: Event::NoteOff {
                            id: self.state.patterns[index].note_ids[track],
                            key: note_state.key,
                            vel: note_state.vel
                        }
                    });

                    free_note(&mut self.state.notes, note_state.id);
                },
                // TODO events for these
                // I'll probably end up removing them or something
                Note::Cut => todo!(),
                Note::Fade => todo!(),

                Note::None => {},

                // rest of the notes
                _ => {
                    let note = event.note as u8;
                    let instrument = if event.instrument == 0 { self.state.patterns[index].last_instrument as usize } else { (event.instrument-1) as usize };
                    self.state.patterns[index].last_instrument = instrument as u8;

                    let old_state = self.state.notes[self.state.patterns[index].note_ids[track]];
                    let id = allocate_note(&mut self.state.notes, note, instrument, self.state.next_note_id);
                    // DAW will allocate on the next ID (if free). We don't want to be using the same ID all over again.
                    self.state.next_note_id = id+1;

                    self.state.patterns[index].note_ids[track] = id;

                    // TODO: replace this with is_free so that plugin APIs like CLAP can notify whenever it's free
                    if self.state.notes[self.state.patterns[index].note_ids[track]].is_on {
                        self.state.event_list.push(TimedEvent {
                            instrument,
                            position: sample_index as u32,
                            event: Event::NoteOff {
                                id: self.state.patterns[index].note_ids[track],
                                key: old_state.key,
                                vel: old_state.vel
                            }
                        });
                    }

                    self.state.event_list.push(TimedEvent {
                        // remember, 0 is none
                        instrument,
                        position: sample_index as u32,
                        event: Event::NoteOn {
                            id,
                            key: note,
                            vel: if event.volume > 127 { 127 } else { event.volume },
                        },
                    })
                },
            }
        }
    }
}

fn format_note(note: Note) -> String {
    match note {
        Note::None => return "...".to_string(),            // none
        Note::PreviousTrack => return "<<<".to_string(), // prev channel
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
