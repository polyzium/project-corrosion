use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use super::{TextCanvas, widgets::Widget};

pub struct TextUI {
    pub canvas: TextCanvas,
    pub widgets: Vec<Box<dyn Widget>>,

    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    sdl_event_pump: sdl2::EventPump,

    wants_to_quit: bool,
}

impl TextUI {
    pub fn new(width: usize, height: usize, font: Vec<u64>, title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .position_centered()
            // .resizable()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        
        Self {
            canvas: TextCanvas::new(
                width,
                height,
                font
            ),
            widgets: Vec::new(),
            sdl_canvas: canvas,
            sdl_event_pump: sdl_context.event_pump().unwrap(),
            wants_to_quit: false,
            
        }
    }

    pub fn event_loop(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    if !repeat {
                        widget_loop(&mut self.widgets, super::events::Event::KeyDown(keycode.unwrap()))
                    } else {
                        widget_loop(&mut self.widgets, super::events::Event::KeyRepeat(keycode.unwrap()))
                    }
                }
                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    widget_loop(&mut self.widgets, super::events::Event::KeyUp(keycode.unwrap()))
                }
                Event::TextInput {
                    timestamp,
                    window_id,
                    text
                } => {
                    widget_loop(&mut self.widgets, super::events::Event::TextInput(text))
                }
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    widget_loop(&mut self.widgets, super::events::Event::MouseMove(x as usize, y as usize))
                }
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    widget_loop(&mut self.widgets, super::events::Event::MouseDown(x as usize, y as usize, mouse_btn))
                }
                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    widget_loop(&mut self.widgets, super::events::Event::MouseUp(x as usize, y as usize, mouse_btn))
                }
                Event::Quit {..} => { self.wants_to_quit = true }
                #[cfg(debug_assertions)]
                e => {
                    println!("{:?}", e);
                }
                #[cfg(not(debug_assertions))]
                _ => {}
            }
        }
    }

    /// Polls events to the widgets and sends drawing commands
    pub fn poll(&mut self) {
        self.event_loop();
        for widget in &mut self.widgets {
            if widget.visible() {
                widget.draw(&self.canvas.get_sender());
            }
        }
    }

    /// Updates the screen based on drawing commands + SDL stuff
    pub fn update(&mut self) {
        self.canvas.update();

        // Create a temporary texture for rendering the buffer
        let texture_creator = self.sdl_canvas.texture_creator();
        let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            self.canvas.width as u32,
            self.canvas.height as u32
        ).unwrap();

        // Copy the contents of the buffer to the texture
        texture.with_lock(None, |buffer: &mut [u8], _: usize| {
            let pixel_data = self.canvas.buffer.as_slice();
            for (i, pixel) in pixel_data.iter().enumerate() {
                let offset = i * 4;
                buffer[offset] = (*pixel & 0xFF) as u8;
                buffer[offset + 1] = ((*pixel >> 8) & 0xFF) as u8;
                buffer[offset + 2] = ((*pixel >> 16) & 0xFF) as u8;
                // buffer[offset + 3] = 0xFF;
            }
        }).unwrap();

        self.sdl_canvas.copy(&texture, None, None).unwrap();
        self.sdl_canvas.present();
        #[cfg(not(debug_assertions))]
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }
}

pub fn widget_loop(widgets: &mut [Box<dyn Widget>], event: super::events::Event) {
    for widget in widgets {
        if widget.visible() && widget.handles_events() {
            widget.handle_event(event.clone());
        }
    }
}