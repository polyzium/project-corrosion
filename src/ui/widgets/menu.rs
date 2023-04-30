use sdl2::keyboard::Keycode;
use crate::{any_impl, ui::{events::Event, theme::{RIM_LIGHT, RIM_DARK}}};
use super::{Widget, Position, fill_region, draw_borders_thin, draw_borders_inner_triangles_thin};

const WIDGET_ID_MENU: u8 = 3;

pub type MenuPage = Vec<MenuItem>;

pub struct MenuItem {
    pub label: String,
    triggered: bool
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            triggered: false,
        }
    }

    pub fn triggered(&mut self) -> bool {
        if self.triggered {
            self.triggered = false;
            true
        } else {
            false
        }
    }
}

pub struct Menu {
    pub pos: Position,
    pub fg: u32,
    pub bg: u32,

    pub pages: Vec<MenuPage>,
    pub selected_item: usize,
    pub current_page: usize,

    pub visible: bool
}

impl Menu {
    pub fn goto_page(&mut self, index: usize) {
        self.current_page = index;
        self.selected_item = 0;
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.current_page = 0;
        self.selected_item = 0;
    }
}

impl Widget for Menu {
    fn type_id(&self) -> u8 {
        WIDGET_ID_MENU
    }

    fn draw(&mut self, canvas_channel: &std::sync::mpsc::Sender<crate::ui::Command>) {
        let mut width = 0usize;

        // Find width of the menu items
        for item in &self.pages[self.current_page] {
            if item.label.len() > width {
                width = item.label.len()
            }
        }

        // Find start pos since self.pos is assumed to be the center
        let start_x = self.pos.x-(width/2);
        let start_y = self.pos.y-(self.pages[self.current_page].len());

        // Draw
        draw_borders_inner_triangles_thin(canvas_channel, Position { x: start_x-1, y: start_y-1 }, Position { x: start_x+width, y: start_y+self.pages[self.current_page].len() }, RIM_LIGHT, RIM_DARK, self.bg);
        fill_region(canvas_channel, Position { x: start_x, y: start_y }, Position { x: start_x+width, y: start_y+self.pages[self.current_page].len() }, self.bg);
        for (i, item) in self.pages[self.current_page].iter().enumerate() {
            canvas_channel.send(crate::ui::Command::Text(start_x, start_y+i, if self.selected_item == i {0xffffff} else {0}, self.bg, item.label.clone()));
        }
    }

    fn handle_event(&mut self, event: crate::ui::events::Event) {
        match event {
            Event::KeyDown(key) | Event::KeyRepeat(key) => {
                match key {
                    Keycode::Up => {
                        if self.selected_item != 0 {
                            self.selected_item -= 1;
                        }
                    },
                    Keycode::Down => {
                        if self.selected_item != self.pages[self.current_page].len()-1 {
                            self.selected_item += 1;
                        }
                    },
                    Keycode::Return => {
                        self.pages[self.current_page][self.selected_item].triggered = true;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn clicked(&mut self) -> bool {
        false
    }

    any_impl!{}

    fn set_visiblity(&mut self, visible: bool) {
        self.visible = visible
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn changed(&mut self) -> bool {
        false
    }

    fn set_handles_events(&mut self, receives: bool) {
        // no-op
    }

    fn handles_events(&self) -> bool {
        true
    }
}