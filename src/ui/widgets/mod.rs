pub mod button;
pub mod pattern_editor;
pub mod container;
pub mod eventhook;
pub mod menu;
pub mod rulers;
pub mod clock;

use std::{sync::mpsc::{Sender, self}, any::Any};

use super::{glyph_indices::*, Command, events::Event};

#[macro_export]
macro_rules! any_impl {
    () => {
        fn as_any(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };
}

#[derive(Clone, Copy)]
pub struct Position{pub x: usize, pub y: usize}
pub trait Widget: Any {
    // Common
    fn type_id(&self) -> u8;
    fn draw(&mut self, canvas_channel: &Sender<super::Command>);
    fn handle_event(&mut self, event: Event);

    // Input
    fn clicked(&mut self) -> bool; // For clickables (buttons, etc). Returns if the widget has been clicked.
    fn changed(&mut self) -> bool; // For arbitrary inputs (text fields, editors, etc). Returns if the widget has changed an underlying value.

    // Visibility
    fn set_visiblity(&mut self, visible: bool);
    fn visible(&self) -> bool;

    // Events
    fn set_handles_events(&mut self, toggle: bool);
    fn handles_events(&self) -> bool;

    // For dyn stuff
    fn as_any(&mut self) -> &mut dyn Any;
}


/// Draws a border around a rectanglular region
#[allow(unused_must_use)]
pub fn draw_borders_thick(canvas_channel: &mpsc::Sender<Command>, pos1: Position, pos2: Position, top_rim: u32, bottom_rim: u32, bg: u32) {
    // Left side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos1.x - 1,
            y,
            top_rim,
            bg,
            BOX_THICK_RIGHT_MIDDLE,
        ));
    }
    // Right side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos2.x,
            y,
            bottom_rim,
            bg,
            BOX_THICK_LEFT_MIDDLE,
        ));
    }
    // Top side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos1.y - 1,
        top_rim,
        bg,
        (BOX_THICK_BOTTOM_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));
    // Bottom side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos2.y,
        bottom_rim,
        bg,
        (BOX_THICK_TOP_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));

    // Top left corner
    canvas_channel.send(Command::Char(pos1.x-1, pos1.y-1, top_rim, bg, CORNER_THICK_BOTTOM_RIGHT));
    // Bottom right corner
    canvas_channel.send(Command::Char(pos2.x, pos2.y, bottom_rim, bg, CORNER_THICK_TOP_LEFT));
    // Top right corner
    canvas_channel.send(Command::Char(pos2.x, pos1.y-1, top_rim, bg, CORNER_THICK_BOTTOM_LEFT));
    // Bottom left corner
    canvas_channel.send(Command::Char(pos1.x-1, pos2.y, top_rim, bg, CORNER_THICK_TOP_RIGHT));
}

/// Draws a border around a rectanglular region. Uses inner glyphs.
#[allow(unused_must_use)]
pub fn draw_borders_inner_triangles_thin(canvas_channel: &mpsc::Sender<Command>, pos1: Position, pos2: Position, top_rim: u32, bottom_rim: u32, bg: u32) {
    // Left side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos1.x,
            y,
            top_rim,
            bg,
            BOX_LEFT_MIDDLE,
        ));
    }
    // Right side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos2.x,
            y,
            bottom_rim,
            bg,
            BOX_RIGHT_MIDDLE,
        ));
    }
    // Top side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos1.y,
        top_rim,
        bg,
        (BOX_TOP_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));
    // Bottom side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos2.y,
        bottom_rim,
        bg,
        (BOX_BOTTOM_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));

    // Top left corner
    canvas_channel.send(Command::Char(pos1.x, pos1.y, top_rim, bg, BOX_TOP_LEFT));
    // Bottom right corner
    canvas_channel.send(Command::Char(pos2.x, pos2.y, bottom_rim, bg, BOX_BOTTOM_RIGHT));
    // Top right corner
    canvas_channel.send(Command::Char(pos2.x, pos1.y, bottom_rim, bg, TRIANGLE_TOPRIGHT));
    // Bottom left corner
    canvas_channel.send(Command::Char(pos1.x, pos2.y, bottom_rim, bg, TRIANGLE_BOTTOMLEFT));
}

#[allow(unused_must_use)]
pub fn draw_borders_thin(canvas_channel: &mpsc::Sender<Command>, pos1: Position, pos2: Position, top_rim: u32, bottom_rim: u32, bg: u32) {
    // Left side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos1.x - 1,
            y,
            top_rim,
            bg,
            BOX_RIGHT_MIDDLE,
        ));
    }
    // Right side
    for y in pos1.y..pos2.y {
        canvas_channel.send(Command::Char(
            pos2.x,
            y,
            bottom_rim,
            bg,
            BOX_LEFT_MIDDLE,
        ));
    }
    // Top side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos1.y - 1,
        top_rim,
        bg,
        (BOX_BOTTOM_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));
    // Bottom side
    canvas_channel.send(Command::Text(
        pos1.x,
        pos2.y,
        bottom_rim,
        bg,
        (BOX_TOP_MIDDLE)
            .to_string()
            .repeat(pos2.x - pos1.x),
    ));

    // Top left corner
    canvas_channel.send(Command::Char(pos1.x-1, pos1.y-1, top_rim, bg, CORNER_BOTTOM_RIGHT));
    // Bottom right corner
    canvas_channel.send(Command::Char(pos2.x, pos2.y, bottom_rim, bg, CORNER_TOP_LEFT));
    // Top right corner
    canvas_channel.send(Command::Char(pos2.x, pos1.y-1, top_rim, bg, CORNER_BOTTOM_LEFT));
    // Bottom left corner
    canvas_channel.send(Command::Char(pos1.x-1, pos2.y, top_rim, bg, CORNER_TOP_RIGHT));
}

#[allow(unused_must_use)]
pub fn fill_region(canvas_channel: &mpsc::Sender<Command>, pos1: Position, pos2: Position, color: u32) {
    for y in pos1.y..pos2.y {
        for x in pos1.x..pos2.x {
            canvas_channel.send(Command::Char(x, y, color, color, ' '));
        }
    }
}