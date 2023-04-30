use std::sync::mpsc::Sender;

use crate::{ui::events::Event, any_impl};

use super::Widget;

// This is a special widget that lets you expose events it receives via channels.
pub struct EventHook {
    pub(crate) tx: Sender<Event>,
    pub enabled: bool,
}

impl Widget for EventHook {
    fn type_id(&self) -> u8 {
        255
    }

    fn draw(&mut self, _: &Sender<crate::ui::Command>) {
        // no-op
    }

    fn handle_event(&mut self, event: Event) {
        if matches!(self.tx.send(event), Err(..)) {
            println!("ui::widgets::eventhook::EventHook - can't send event!")
        };
    }

    fn clicked(&mut self) -> bool {
        false
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
        self.enabled = toggle
    }

    fn handles_events(&self) -> bool {
        self.enabled
    }
}