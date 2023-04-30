use sdl2::{keyboard::Keycode, mouse::MouseButton};

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown(Keycode),
    KeyRepeat(Keycode),
    KeyUp(Keycode),
    TextInput(String),
    MouseMove(usize, usize),
    MouseDown(usize, usize, MouseButton),
    MouseUp(usize, usize, MouseButton),
}