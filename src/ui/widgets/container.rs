use crate::any_impl;

use super::Widget;

const WIDGET_ID_CONTAINER: u8 = 2;

pub struct Container {
    pub(crate) handles_events: bool,
    pub widgets: Vec<Box<dyn Widget>>
}

impl Widget for Container {
    fn type_id(&self) -> u8 {
        WIDGET_ID_CONTAINER
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        for widget in &mut self.widgets {
            widget.draw(canvas_channel);
        }
    }

    fn handle_event(&mut self, event: crate::ui::events::Event) {
        for widget in &mut self.widgets {
            widget.handle_event(event.clone())
        }
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
        self.handles_events = toggle
    }

    fn handles_events(&self) -> bool {
        self.handles_events
    }
}

pub struct PagerContainer {
    pub handles_events: bool,
    pub widgets: Vec<Box<dyn Widget>>,
    pub current_widget: usize
}

impl Widget for PagerContainer {
    fn type_id(&self) -> u8 {
        WIDGET_ID_CONTAINER
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        self.widgets[self.current_widget].draw(canvas_channel)
    }

    fn handle_event(&mut self, event: crate::ui::events::Event) {
        self.widgets[self.current_widget].handle_event(event)
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
        self.handles_events = toggle
    }

    fn handles_events(&self) -> bool {
        self.handles_events
    }
}