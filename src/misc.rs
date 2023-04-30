use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use crate::engine::pattern::Note;

#[macro_export]
macro_rules! get_widget_mut {
    ($widget:expr, $type:ty) => {
        ($widget.as_any().downcast_mut().unwrap()) as &mut $type
    };
}

#[macro_export]
macro_rules! last_index {
    ($indexable:ident) => {
        $indexable.len()-1
    };
}

#[macro_export]
macro_rules! handle_menu {
    ($item:expr, $handler:block) => {
        if $item.triggered() $handler
    };
}

fn longest_line_len(string: &str) -> usize {
    let mut len_so_far = 0usize;

    for line in string.lines() {
        if line.len() > len_so_far {
            len_so_far = line.len();
        }
    }

    len_so_far
}