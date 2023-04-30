pub mod glyph_indices;
pub mod theme;
pub mod widgets;
pub mod combined;
pub mod events;

use std::{
    fmt::Write,
    fs::File,
    io::Read, rc::Rc, borrow::BorrowMut, cell::Cell, sync::mpsc::{self, Receiver, Sender}
};

use crate::ui::glyph_indices::*;

use self::{
    glyph_indices::BOX_LEFT_MIDDLE,
};

// 8x8 glyphs are 8 bytes long
type Glyph = u64;

pub enum Command {
    Char(usize, usize, u32, u32, char),
    Text(usize, usize, u32, u32, String),
    // Rectangle(usize, usize, usize, usize, u32)
}

pub struct TextCanvas {
    pub width: usize,
    pub height: usize,

    pub buffer: Vec<u32>,
    pub font: Vec<Glyph>,

    channel: (Sender<Command>, Receiver<Command>)
}

pub fn render_slider_block(progress: f32, channel: &Sender<Command>, x: usize, y: usize) {
    let mut s = String::new();

    let progress_percent = (progress * 100.0) as usize;
    let left_blocks = (progress_percent / 10) as usize;
    let right_blocks = 10 - left_blocks;

    s.push(SLIDER_BLOCK_LEFT1);

    for i in 0..left_blocks {
        let char_to_print = match i {
            0 => SLIDER_BLOCK_LEFT1,
            1 => SLIDER_BLOCK_LEFT2,
            2 => SLIDER_BLOCK_LEFT3,
            3 => SLIDER_BLOCK_LEFT4,
            4 => SLIDER_BLOCK_LEFT5,
            5 => SLIDER_BLOCK_LEFT6,
            6 => SLIDER_BLOCK_LEFT7,
            _ => SLIDER_BLOCK_LEFT8,
        };

        s.push(char_to_print);
    }

    for _ in 0..right_blocks {
        s.push(SLIDER_BLOCK_RIGHT1);
    }

    channel.send(Command::Text(x, y, 0xffffff, 0, s));
}

/// Convert pixel space coordinates to character space
pub fn pixel_to_char(x: usize, y: usize) -> (usize, usize) {
    (x / 8, y / 8)
}

/// Load a font from Impulse/Schism Tracker ITF file
pub fn load_font_from_itf(mut itffile: File) -> Vec<u64> {
    let mut glyphs: Vec<u64> = vec![0; 130042];

    let mut itfdata: Vec<u8> = Vec::with_capacity(2048);
    itffile.read_to_end(&mut itfdata).unwrap();

    let itfglyphs = itfdata
        .chunks_exact(8)
        .map(|chunk| {
            u64::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            ])
            .reverse_bits()
        })
        .collect::<Vec<u64>>();

    for (i, glyph) in itfglyphs.iter().enumerate() {
        glyphs[i] = *glyph
    }

    glyphs
}

/// Load Unifont HEX file
pub fn load_font_from_hex(mut hexfile: File) -> Vec<u64> {
    let mut glyphs: Vec<u64> = vec![0; 130042];
    let mut hexdata = String::new();

    hexfile.read_to_string(&mut hexdata).unwrap();
    let mut hexadecimal_str = String::new();

    let mut index: usize = 0;
    let mut data: u64;

    for char in hexdata.chars() {
        match char {
            ':' => {
                index = usize::from_str_radix(&hexadecimal_str, 16).unwrap();
                hexadecimal_str.clear();
            }
            '\n' => {
                data = u64::from_str_radix(&hexadecimal_str, 16).unwrap();
                hexadecimal_str.clear();

                glyphs[index] = data.reverse_bits();
            }
            _ => hexadecimal_str.write_char(char).unwrap(),
        }
    }

    glyphs
}

impl TextCanvas {
    pub fn new(w: usize, h: usize, font: Vec<u64>) -> Self {
        let buffer: Vec<u32> = vec![0; w * h];

        TextCanvas {
            width: w,
            height: h,

            buffer,
            font,
            channel: mpsc::channel()
        }
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.channel.0.clone()
    }

    pub fn update(&mut self) {
        loop {
            let cmd_recv = self.channel.1.try_recv();
            match cmd_recv {
                Ok(cmd) => {
                    match cmd {
                        Command::Char(x, y, fg, bg, char) => self.char(x, y, fg, bg, char),
                        Command::Text(x, y, fg, bg, string) => self.text(x, y, fg, bg, &string),
                        /* Command::Rectangle(x1, y1, x2, y2, _) => {

                        } */
                    }
                },
                Err(_) => break,
            }
        }
    }

    pub fn char(&mut self, x: usize, y: usize, fg: u32, bg: u32, char: char) {
        let (mut i, mut j): (usize, usize) = (x * 8, y * 8);

        for bit in 0..64 {
            self.buffer[j * (self.width) + i] = if self.font[char as usize] & (1 << bit) != 0 {
                fg
            } else {
                bg
            };

            i += 1;

            if i > (x * 8) + 7 {
                // Out of bounds
                j += 1; // Vertical down
                i = x * 8; // Reset x
            }
        }
    }

    /// Draw text in character space
    pub fn text(&mut self, mut x: usize, mut y: usize, fg: u32, bg: u32, string: &str) {
        let x_initial = x;

        for (_, char) in string.char_indices() {
            match char {
                '\n' => {
                    y += 1;
                    x = x_initial
                }
                _ => {
                    self.char(x, y, fg, bg, char);
                    x += 1;
                }
            }
        }
    }
}
