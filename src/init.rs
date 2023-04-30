use std::collections::HashMap;

use sdl2::keyboard::Keycode;

use crate::{ui::{widgets::{menu::{Menu, MenuPage, MenuItem}, Position}, theme::MAIN_COLOR}, WIDTH, HEIGHT, engine::pattern::Note};

pub const MENU_MAIN: usize = 0;
pub const MENU_FILE: usize = 1;
pub const MENU_PLAYBACK: usize = 2;
pub const MENU_SETTINGS: usize = 3;

pub fn main_menu() -> Menu {
    let mut pages: Vec<MenuPage> = Vec::with_capacity(16);

    pages.push(vec![
        MenuItem::new("File"),
        MenuItem::new("Playback"),
        MenuItem::new("Settings"),
        MenuItem::new("Quit")
    ]);

    // File menu
    pages.push(vec![
        MenuItem::new("Not implemented, sorry!")
    ]);

    // Playback menu
    pages.push(vec![
        MenuItem::new("Play"),
        MenuItem::new("Pause"),
        MenuItem::new("Stop")
    ]);

    // Settings menu
    pages.push(vec![
        MenuItem::new("Not implemented, sorry!")
    ]);

    Menu {
        pages,
        selected_item: 0,
        current_page: 0,
        visible: false,
        pos: Position { x: WIDTH/16, y: HEIGHT/16 },
        fg: 0,
        bg: MAIN_COLOR,
    }
}

macro_rules! insert_key {
    ($map:ident, $key:ident, $note:ident) => {
        $map.insert(Keycode::$key, Note::$note)
    };
}

pub fn default_kbd_mapping() -> HashMap<Keycode, Note> {
    let mut map: HashMap<Keycode, Note> = HashMap::new();

    // Upper half
    insert_key!(map, Q, C5);
    insert_key!(map, Num2, Cs5);
    insert_key!(map, W, D5);
    insert_key!(map, Num3, Ds5);
    insert_key!(map, E, E5);
    insert_key!(map, R, F5);
    insert_key!(map, Num5, Fs5);
    insert_key!(map, T, G5);
    insert_key!(map, Num6, Gs5);
    insert_key!(map, Y, A5);
    insert_key!(map, Num7, As5);
    insert_key!(map, U, B5);

    insert_key!(map, I, C6);
    insert_key!(map, Num9, Cs6);
    insert_key!(map, O, D6);
    insert_key!(map, Num0, Ds6);
    insert_key!(map, P, E6);

    // Lower half
    insert_key!(map, Z, C4);
    insert_key!(map, S, Cs4);
    insert_key!(map, X, D4);
    insert_key!(map, D, Ds4);
    insert_key!(map, C, E4);
    insert_key!(map, V, F4);
    insert_key!(map, G, Fs4);
    insert_key!(map, B, G4);
    insert_key!(map, H, Gs4);
    insert_key!(map, N, A4);
    insert_key!(map, J, As4);
    insert_key!(map, M, B4);

    map
}