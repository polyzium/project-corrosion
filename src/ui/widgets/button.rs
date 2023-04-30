use std::sync::mpsc::Sender;
use crate::{ui::{glyph_indices::*, Command, pixel_to_char, events::Event}, any_impl};

use super::{Widget, Position};

const WIDGET_ID_BUTTON: u8 = 0;

pub struct Button {
    pub pos: Position,
    pub label: String,

    pub fg_bg: (u32, u32),
    pub rims: (u32, u32),

    pressed: bool,
    hovered: bool,
    last_mouse_pos: Option<(usize, usize)>,

    clicked: bool,
}

impl Button {
    pub fn new(
        pos: Position,
        label: String,
        fg_bg: (u32, u32),
        rims: (u32, u32),
    ) -> Self {
        Button {
            pos,
            label,

            fg_bg,
            rims,

            pressed: false,
            hovered: false,
            last_mouse_pos: None,

            clicked: false,
        }
    }
}

impl Widget for Button {
    fn draw(&mut self, canvas_channel: &Sender<Command>) {
        let mut rim_top = self.rims.0;
        let mut rim_bottom = self.rims.1;

        if self.pressed {
            (rim_top, rim_bottom) = (rim_bottom, rim_top);
        }

        canvas_channel.send(Command::Text(
            self.pos.x,
            self.pos.y,
            self.fg_bg.0,
            self.fg_bg.1,
            self.label.to_string(),
        ));

        // Left side
        canvas_channel.send(Command::Char(
            self.pos.x - 1,
            self.pos.y,
            rim_top,
            self.fg_bg.1,
            BOX_RIGHT_MIDDLE,
        ));
        // Right side
        canvas_channel.send(Command::Char(
            self.pos.x + self.label.len(),
            self.pos.y,
            rim_bottom,
            self.fg_bg.1,
            BOX_LEFT_MIDDLE,
        ));
        // Top side
        canvas_channel.send(Command::Text(
            self.pos.x,
            self.pos.y - 1,
            rim_top,
            self.fg_bg.1,
            (BOX_BOTTOM_MIDDLE)
                .to_string()
                .repeat(self.label.len()),
        ));
        // Bottom side
        canvas_channel.send(Command::Text(
            self.pos.x,
            self.pos.y + 1,
            rim_bottom,
            self.fg_bg.1,
            (BOX_TOP_MIDDLE)
                .to_string()
                .repeat(self.label.len()),
        ));

        // Top left corner
        canvas_channel.send(Command::Char(self.pos.x-1, self.pos.y-1, rim_top, self.fg_bg.1, CORNER_BOTTOM_RIGHT));
        // Bottom right corner
        canvas_channel.send(Command::Char(self.pos.x + self.label.len(), self.pos.y+1, rim_bottom, self.fg_bg.1, CORNER_TOP_LEFT));
        // Top right corner
        canvas_channel.send(Command::Char(self.pos.x + self.label.len(), self.pos.y-1, rim_bottom, self.fg_bg.1, CORNER_BOTTOM_LEFT));
        // Bottom left corner
        canvas_channel.send(Command::Char(self.pos.x-1, self.pos.y+1, rim_bottom, self.fg_bg.1, CORNER_TOP_RIGHT));
    }

    fn handle_event(&mut self, event: Event) {
        let mut real_pos: Position = Position { x: 0, y: 0 };

        match event {
            Event::MouseMove(x, y) => (real_pos.x, real_pos.y) = (x, y),
            Event::MouseDown(x, y, _) => {
                (real_pos.x, real_pos.y) = (x, y);
                self.last_mouse_pos = Some(pixel_to_char(real_pos.x, real_pos.y));
            }
            Event::MouseUp(x, y, _) => {
                (real_pos.x, real_pos.y) = (x, y);
            }
            _ => {}
        }

        let mouse_pos = pixel_to_char(real_pos.x, real_pos.y);

        let mouse_in_bounds = (self.pos.x..self.pos.x + self.label.len()).contains(&mouse_pos.0)
            && mouse_pos.1 == self.pos.y;

        if mouse_in_bounds {
            self.hovered = true;

            match event {
                Event::MouseDown(..) => self.pressed = true,
                Event::MouseUp(..) => {
                    if self.pressed {
                        self.pressed = false;
                        self.clicked = true;
                    }
                }
                _ => {}
            }
        } else {
            self.pressed = false;
            self.hovered = false;
        }
    }

    fn type_id(&self) -> u8 {
        WIDGET_ID_BUTTON
    }

    fn clicked(&mut self) -> bool {
        if self.clicked {
            self.clicked = false;
            true
        } else {
            false
        }
    }

    any_impl!{}

    fn set_visiblity(&mut self, visible: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        true
    }

    fn changed(&mut self) -> bool {
        false
    }

    fn set_handles_events(&mut self, toggle: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        true
    }
}
