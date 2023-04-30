use crate::{any_impl, ui::glyph_indices::{DOTTED_LINE, BOX_TOP_MIDDLE}};

use super::Widget;

const WIDGET_ID_LABELRULER: u8 = 4;
const WIDGET_ID_VSEPARATOR: u8 = 5;

pub struct LabelRuler {
    pub label: String,

    pub bg_color: u32,
    pub ruler_color: u32,
    pub label_color: u32,

    pub start_x: usize,
    pub end_x: usize,
    pub y: usize
}

#[allow(unused_must_use)]
impl Widget for LabelRuler {
    fn type_id(&self) -> u8 {
        WIDGET_ID_LABELRULER
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        for x in self.start_x..self.end_x {
            canvas_channel.send(crate::ui::Command::Char(x, self.y, self.ruler_color, self.bg_color, DOTTED_LINE));
        }

        canvas_channel.send(crate::ui::Command::Text(((self.start_x+self.end_x)/2)-(self.label.len()/2), self.y, self.label_color, self.bg_color, " ".to_owned()+&self.label+" "));
    }

    fn handle_event(&mut self, _: crate::ui::events::Event) {
        // no-op
    }

    fn clicked(&mut self) -> bool {
        false
    }

    fn changed(&mut self) -> bool {
        false
    }

    fn set_visiblity(&mut self, _: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        true
    }

    any_impl!{}

    fn set_handles_events(&mut self, receives: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        false
    }
}

pub struct VerticalSeparator {
    pub bg_color: u32,
    pub ruler_color: u32,

    pub start_x: usize,
    pub end_x: usize,
    pub y: usize
}

#[allow(unused_must_use)]
impl Widget for VerticalSeparator {
    fn type_id(&self) -> u8 {
        WIDGET_ID_VSEPARATOR
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        for x in self.start_x..self.end_x {
            canvas_channel.send(crate::ui::Command::Char(x, self.y, self.ruler_color, self.bg_color, DOTTED_LINE));
        }
    }

    fn handle_event(&mut self, _: crate::ui::events::Event) {
        // no-op
    }

    fn clicked(&mut self) -> bool {
        false
    }

    fn changed(&mut self) -> bool {
        false
    }

    fn set_visiblity(&mut self, _: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        true
    }

    any_impl!{}

    fn set_handles_events(&mut self, receives: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        false
    }
}