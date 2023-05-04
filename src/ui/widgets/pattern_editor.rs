use std::{sync::mpsc, collections::HashMap};
use sdl2::keyboard::Keycode;

use crate::{ui::{Command, events::Event, glyph_indices::{CENTERED_BORDER, CENTERED_DOT_THIN}, pixel_to_char}, engine::{pattern::{Pattern, Note}, state::PatternState}, any_impl};

use super::{Widget, Position, draw_borders_thick, fill_region};

const WIDGET_ID_PATTERNVIEW: u8 = 1;

const COLUMN_NOTE: u8 = 0;
const COLUMN_INSTRUMENT: u8 = 1;
const COLUMN_VOLUME: u8 = 2;
// const COLUMN_EFFECT: u8 = 3;
// const COLUMN_EFFECTVALUE: u8 = 4;

fn push_digit<T: num::Integer+std::fmt::Display>(num: T, digit: u8) -> T {
    let mut num_string = num.to_string();
    let digit_string = digit.to_string();
    num_string.push_str(&digit_string);
    T::from_str_radix(&num_string, 10).ok().unwrap()
}

fn truncate_number<T: num::Integer+std::fmt::Display>(num: T, digits: usize) -> T {
    let num_string = num.to_string();
    let len = num_string.len();

    if len <= digits {
        return num;
    }

    let truncated_str = &num_string[(len - digits)..len];
    T::from_str_radix(&truncated_str, 10).ok().unwrap()
}

pub struct PatternEditor {
    pos1: Position,
    pos2: Position,
    focused: bool,
    ctrl_held: bool,
    changed: bool,
    temp_volume: u16,

    pub pattern: Option<Pattern>,
    pub state: Option<PatternState>,
    pub key_mapping: HashMap<Keycode, Note>,

    pub text_color: u32,
    pub outer_bg: u32,
    pub inner_bg: u32,
    pub top_rim: u32,
    pub bottom_rim: u32,

    pub beat_color: u32,
    pub bar_color: u32,

    pub row_selection_color: u32,
    pub column_selection_color: u32,

    current_track: usize,
    current_column: u8,
    current_row: usize,

    track_scroll: usize,
    row_scroll: usize
}

impl PatternEditor {
    pub fn new(pos1: Position, pos2: Position) -> Self {
        Self {
            pos1,
            pos2,
            focused: false,
            ctrl_held: false,
            changed: false,
            temp_volume: 65535u16,

            text_color: 0,
            outer_bg: 0,
            inner_bg: 0,
            top_rim: 0,
            bottom_rim: 0,

            beat_color: 0x082838,
            bar_color: 0x381c08,

            row_selection_color: 0x3f3f3f,
            column_selection_color: 0x7f7f7f,

            current_track: 0,
            current_column: 0,
            current_row: 0,

            pattern: None,
            state: None,
            key_mapping: HashMap::new(),

            track_scroll: 0,
            row_scroll: 0,
        }
    }
}

macro_rules! temp_volume_get {
    ($self:ident) => {
        $self.temp_volume = if $self.pattern.as_ref().unwrap().rows[$self.current_row][$self.current_track].volume as u16 > 127 {
            65535
        } else {
            $self.pattern.as_ref().unwrap().rows[$self.current_row][$self.current_track].volume as u16
        };
    };
}

macro_rules! temp_volume_set {
    ($self:ident) => {
        $self.pattern.as_mut().unwrap().rows[$self.current_row][$self.current_track].volume = if $self.temp_volume > 999 {
            128
        } else {
            $self.temp_volume.clamp(0, 127) as u8
        };
    };
}

#[allow(unused_must_use)]
impl Widget for PatternEditor {
    fn draw(&mut self, canvas_channel: &mpsc::Sender<Command>) {
        draw_borders_thick(canvas_channel, self.pos1, self.pos2, self.top_rim, self.bottom_rim, self.outer_bg);

        let mut x: usize;
        let mut y: usize;

        y = self.pos1.y;

        // Fill the widget with bg color
        for j in self.pos1.y..self.pos2.y {
            for i in self.pos1.x..self.pos2.x {
                canvas_channel.send(Command::Char(i, j, 0, 0, ' '));
            }
        }

        if matches!(self.pattern, None) {
            canvas_channel.send(Command::Text(self.pos1.x, self.pos1.y, self.text_color, self.inner_bg, "self.pattern is None!\nCongrats, you probably found a bug.".to_string()));
            return;
        }

        for xr in self.pos1.x..self.pos2.x {
            if (self.pos1.y..self.pos2.y).contains(&(self.pos1.y+(self.current_row.saturating_sub(self.row_scroll)))) {
                canvas_channel.send(Command::Char(xr, self.pos1.y+(self.current_row.saturating_sub(self.row_scroll)), 0, self.row_selection_color, ' '));
            }
        }

        for i in self.row_scroll..self.pattern.as_ref().unwrap().rows.len() {
            let row = &self.pattern.as_ref().unwrap().rows[i];

            /* if self.current_row == i {
                for xr in self.pos1.x..self.pos2.x {
                    canvas_channel.send(Command::Char(xr, self.pos1.y+(self.current_row-self.row_scroll), 0, self.row_selection_color, ' '));
                }
            } */

            x = self.pos1.x;

            let number_bg = if self.state.as_ref().unwrap().playing && i == self.state.as_ref().unwrap().row.into() {
                0xffffff
            } else {
                0
            };
            canvas_channel.send(Command::Text(x-4, y, number_bg, self.outer_bg, format!("{:0>3}", i)));

            let row_bg = if i%(self.pattern.as_ref().unwrap().rpb as usize*4) == 0 {
                if self.current_row != i {
                    fill_region(canvas_channel, Position { x: self.pos1.x, y: (self.pos1.y+i)-self.row_scroll }, Position { x: self.pos2.x, y: (self.pos1.y+i+1)-self.row_scroll }, self.bar_color);
                }
                self.bar_color
            } else if i%self.pattern.as_ref().unwrap().rpb as usize == 0 {
                if self.current_row != i {
                    fill_region(canvas_channel, Position { x: self.pos1.x, y: (self.pos1.y+i)-self.row_scroll }, Position { x: self.pos2.x, y: (self.pos1.y+i+1)-self.row_scroll }, self.beat_color);
                }
                self.beat_color
            } else {
                self.inner_bg
            };

            let mut track_counter = 0;
            for j in self.track_scroll..row.len() {
                let track = &row[j];

                if i == self.row_scroll {
                    canvas_channel.send(Command::Text(self.pos1.x+(track_counter*11), self.pos1.y-1, 0xffffff, self.top_rim, format!(" Track {:0>2} ", j+1)));
                    track_counter += 1;
                }

                let bg = if i == self.current_row && j == self.current_track.into() { self.column_selection_color } else if i == self.current_row { self.row_selection_color } else { row_bg };

                let note_string = format_note(track.note);
                let instr_string = if track.instrument != 0 { format!("{:0>2}", track.instrument) } else { CENTERED_DOT_THIN.to_string().repeat(2) };
                let vol_string = if track.volume <= 127 { format!("{:0>3}", track.volume) } else { CENTERED_DOT_THIN.to_string().repeat(3) };

                canvas_channel.send(Command::Text(x, y, self.text_color, if self.current_column == COLUMN_NOTE { bg } else if i == self.current_row { self.row_selection_color } else { row_bg }, note_string));
                canvas_channel.send(Command::Text(x+4, y, self.text_color, if self.current_column == COLUMN_INSTRUMENT { bg } else if i == self.current_row { self.row_selection_color } else { row_bg }, instr_string));

                if self.current_row == i && self.current_track == j && self.current_column == COLUMN_VOLUME {
                    canvas_channel.send(Command::Text(x+7, y, if self.temp_volume > 999 {self.text_color} else if self.temp_volume > 127 {0xff0000} else {self.text_color}, if self.current_column == COLUMN_VOLUME { bg } else if i == self.current_row { self.row_selection_color } else { row_bg }, format!("{:0>3}", if self.temp_volume > 999 {CENTERED_DOT_THIN.to_string().repeat(3)} else {self.temp_volume.to_string()} )));
                } else {
                    canvas_channel.send(Command::Text(x+7, y, self.text_color, if self.current_column == COLUMN_VOLUME { bg } else if i == self.current_row { self.row_selection_color } else { row_bg }, vol_string));
                }

                canvas_channel.send(Command::Char(x+10, y, self.outer_bg, if i == self.current_row { self.row_selection_color } else { row_bg }, CENTERED_BORDER));

                x += 11;
                // If out of bounds to the right
                if (x+11) >= self.pos2.x {
                    // If the out of bounds track is selected, scroll to the right
                    if self.current_track == j+1 {
                        self.track_scroll += 1;
                    }

                    // Stop rendering
                    break;
                }

                // If out of bounds to the left, scroll to the left
                if self.current_track < self.track_scroll {
                    self.track_scroll -= 1;
                }
            }

            y += 1;
            if y >= self.pos2.y {

                // Stop rendering
                break;
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown(key) | Event::KeyRepeat(key) => {
                if matches!(key as u32, 0x4000004F..=0x40000052) { // arrows
                    temp_volume_set!(self);
                }

                match key {
                    Keycode::Up => {
                        if self.current_row != 0 {
                            self.current_row -= 1;
                        }
                    },
                    Keycode::Down => {
                        if self.current_row != self.pattern.as_ref().unwrap().rows.len()-1 {
                            self.current_row += 1;
                        }
                    },
                    Keycode::Left => {
                        if self.current_column == COLUMN_NOTE {
                            if self.current_track != 0 {
                                self.current_track -= 1;
                                self.current_column = COLUMN_VOLUME;
                            }
                        } else {
                            if self.ctrl_held && self.current_track != 0 {
                                self.current_track -= 1;
                            } else {
                                self.current_column -= 1;
                            }
                        }
                    },
                    Keycode::Right => {
                        if self.current_column == COLUMN_VOLUME {
                            // All rows have the same amount of tracks/events, so just pick the first one
                            if self.current_track != self.pattern.as_ref().unwrap().rows[0].len()-1 {
                                self.current_track += 1;
                                self.current_column = COLUMN_NOTE;
                            }
                        } else {
                            if self.ctrl_held && self.current_track != self.pattern.as_ref().unwrap().rows[0].len()-1 {
                                self.current_track += 1;
                            } else {
                                self.current_column += 1;
                            }
                        }
                    },
                    Keycode::PageDown => {
                        let row_capacity = self.pos2.y-self.pos1.y;
                        let pat = self.pattern.as_ref().unwrap();

                        self.current_row += row_capacity-1;
                        if self.current_row > pat.rows.len()-1 {
                            self.current_row = pat.rows.len()-1
                        }
                    },
                    Keycode::PageUp => {
                        let row_capacity = self.pos2.y-self.pos1.y;
                        // let pat = self.pattern.as_ref().unwrap();

                        self.current_row = self.current_row.saturating_sub(row_capacity-1);
                        /* if self.current_row >  {
                            self.current_row = pat.rows.len()-1
                        } */
                    },
                    Keycode::LCtrl | Keycode::RCtrl => {
                            self.ctrl_held = true;
                    },
                    Keycode::Period | Keycode::Delete => {
                        let event = &mut self.pattern.as_mut().unwrap()
                        .rows[self.current_row][self.current_track];

                        match self.current_column {
                            COLUMN_NOTE => {
                                event.note = Note::None;
                            },
                            COLUMN_INSTRUMENT => {
                                event.instrument = 0;
                            },
                            COLUMN_VOLUME => {
                                event.volume = 128;
                                self.temp_volume = 65535;
                            },
                            _ => {}
                        }

                        self.changed = true;
                    }
                    _ => {
                        match self.current_column {
                            COLUMN_NOTE => {
                                let pat = self.pattern.as_mut().unwrap();

                                if let Some(note) = self.key_mapping.get(&key) {
                                    pat.rows[self.current_row][self.current_track].note = *note;
                                    self.changed = true;
                                } else {
                                    match key {
                                        Keycode::Backquote => {
                                            pat.rows[self.current_row][self.current_track].note = Note::Off;
                                            self.changed = true;
                                        }
                                        _ => {}
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }

                let row_capacity = self.pos2.y-self.pos1.y;
                if self.current_row > self.row_scroll+row_capacity-1 {
                    self.row_scroll = self.current_row-(row_capacity-1);
                } else if self.current_row < self.row_scroll {
                    self.row_scroll = self.current_row;
                }

                if matches!(key as u32, 0x4000004F..=0x40000052) { // arrows
                    temp_volume_get!(self);
                    self.changed = true;
                }
            },
            Event::KeyUp(key) => {
                match key {
                    Keycode::LCtrl | Keycode::RCtrl => {
                        self.ctrl_held = false;
                    },
                    _ => {}
                }
            }
            Event::MouseMove(_, _) => {},
            Event::MouseDown(x, y, _) => {
                let (nx, ny) = pixel_to_char(x, y);

                let in_bounds = (self.pos1.x..=self.pos2.x).contains(&nx) && (self.pos1.y..=self.pos2.y).contains(&ny);
                self.focused = in_bounds;
            },
            Event::MouseUp(_, _, _) => {},
            Event::TextInput(text) => {
                for char in text.chars() {
                    if char.is_numeric() {
                        match self.current_column {
                            COLUMN_INSTRUMENT => {
                                let event = &mut self.pattern.as_mut().unwrap().rows[self.current_row][self.current_track];
                                let tmp: u16 = push_digit(event.instrument as u16, (char as u8)-48);
                                event.instrument = truncate_number(tmp, 2) as u8;

                                self.changed = true;
                            },
                            COLUMN_VOLUME => {
                                if self.temp_volume == 65535 {
                                    self.temp_volume = 0;
                                }

                                let tmp: u16 = push_digit(self.temp_volume, (char as u8)-48);
                                self.temp_volume = truncate_number(tmp, 3);

                                if self.temp_volume <= 127 {
                                    let event = &mut self.pattern.as_mut().unwrap().rows[self.current_row][self.current_track];

                                    event.volume = self.temp_volume as u8;
                                    self.changed = true;
                                }
                            },
                            _ => {}
                        }
                    }
                }
            },
        }
    }

    fn type_id(&self) -> u8 {
        WIDGET_ID_PATTERNVIEW
    }

    fn clicked(&mut self) -> bool {
        false
    }

    any_impl!{}

    fn set_visiblity(&mut self, _: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        true
    }

    fn changed(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            true
        } else {
            false
        }
    }

    fn set_handles_events(&mut self, _: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        true
    }
}

fn format_note(note: Note) -> String {
    match note {
        Note::None => return CENTERED_DOT_THIN.to_string().repeat(3),            // none
        Note::PreviousTrack => return "<<<".to_string(),   // prev channel
        Note::Off => return "===".to_string(),             // off
        Note::Cut => return "^^^".to_string(),             // cut/choke
        Note::Fade => return "~~~".to_string(),            // fade
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