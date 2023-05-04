use crate::any_impl;

use super::{Widget, Position};

const WIDGET_ID_CLOCK: u8 = 7;

pub struct Clock {
    pub pos: Position,

    pub bg_color: u32,
    pub color: u32,
    pub beat_color: u32,
    pub downbeat_color: u32,

    pub ticks: u32,
    pub ppq: u16,
    pub flash_on_beat: bool,
    pub playing: bool,
}

#[allow(unused_must_use)]
impl Widget for Clock {
    fn type_id(&self) -> u8 {
        WIDGET_ID_CLOCK
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        canvas_channel.send(crate::ui::Command::Text(
            self.pos.x, self.pos.y, self.color, self.bg_color,
            format!("{:>3}:{:0>2}:{:0>3}",
                (self.ticks/(self.ppq as u32)/4)+1,
                (self.ticks/(self.ppq as u32/4)%16)+1,
                self.ticks % (self.ppq as u32)
            )
        ));
    }

    fn handle_event(&mut self, event: crate::ui::events::Event) {
        // no-op
        // TODO possibly add B:S:T and M:S:CS switch via a mouse click
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

    fn set_handles_events(&mut self, _: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        false
    }

    any_impl!{}
}