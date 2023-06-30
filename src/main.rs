use std::fs::File;
use std::mem;
use std::panic::catch_unwind;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use cpal::{SampleRate, BufferSize};
use engine::pattern::Pattern;
use sdl2::keyboard::Keycode;
use ui::widgets::pattern_editor::PatternEditor;
use ui::widgets::{Position, fill_region};
use ui::widgets::button::Button;
use crate::init::{main_menu, default_kbd_mapping};
use crate::ui::Command;
use crate::ui::widgets::clock::Clock;
use crate::ui::widgets::container::{PagerContainer, Container};
use crate::ui::widgets::{draw_borders_inner_triangles_thin, Widget};
use crate::ui::widgets::eventhook::EventHook;
use crate::ui::widgets::menu::Menu;
use crate::ui::widgets::rulers::LabelRuler;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BufferSize::Fixed;
use ui::combined::TextUI;
use ui::theme::*;
use ui::{load_font_from_hex, load_font_from_itf};

mod engine;
pub mod ui;
mod misc;
mod init;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

#[allow(unused_must_use)]
fn main() {
    // Set panic to display a native dialog
    std::panic::set_hook(Box::new(|panic_data| {
        let payload_raw = panic_data.payload();

        let payload_msg = if let Some(payload) = payload_raw.downcast_ref::<&str>() {
            format!("{}\n({})", payload, panic_data.location().unwrap())
        } else if let Some(payload) = payload_raw.downcast_ref::<String>() {
            format!("{}\n({})", payload, panic_data.location().unwrap())
        } else {
            format!("Panic occurred with unknown payload")
        };

        native_dialog::MessageDialog::new()
        .set_type(native_dialog::MessageType::Error)
        .set_title("PANIC!")
        .set_text(&format!("{}\n\nProject Corrosion will now close.", payload_msg))
        .show_alert();
    }));

    // Audio
    let mut samplerate = 48000;
    let mut channels = 2u8;
    let mut buffer_size = 512u32;

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output devices available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("Unable to query configs");
    for c in device.supported_output_configs().unwrap() {
        println!("{:?}", c);
    }

    let mut config: cpal::StreamConfig;

    #[cfg(not(target_os = "windows"))]
    {
        config = supported_configs_range
            .find(|conf| conf.channels() == channels.into())
            .expect(&format!("Unable to find a {}-channel config", channels))
            .with_sample_rate(cpal::SampleRate(samplerate))
            .into();
    }

    // Fix for windows
    #[cfg(target_os = "windows")]
    {
        let found_config = supported_configs_range
        .find(|conf| conf.channels() == channels.into() && conf.max_sample_rate() == cpal::SampleRate(samplerate) );
        
        if found_config.is_some() {
            config = found_config.unwrap().with_max_sample_rate().into();
        } else {
            native_dialog::MessageDialog::new()
            .set_type(native_dialog::MessageType::Warning)
            .set_title("Configuration")
            .set_text(&format!("Unable to find a {samplerate} Hz {channels}-channel audio configuration.\nUsing first available configuration instead.\n\nThis may cause issues. Please switch to a supported configuration whenever possible."))
            .show_alert();

            // The iterator must've consumed all elements at this point
            supported_configs_range = device
            .supported_output_configs()
            .expect("Unable to query configs");

            config = supported_configs_range
            .find(|conf| { return true })
            .expect(&format!("No configs!?"))
            .with_max_sample_rate()
            .into();
        }

        samplerate = config.sample_rate.0;
        channels = config.channels as u8;
    }

    config.buffer_size = BufferSize::Fixed(buffer_size);

    let daw = Arc::new(Mutex::new(engine::DAWEngine::new(samplerate, channels, buffer_size)));
    let daw_a = daw.clone();

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut locked_daw = daw_a.lock().expect("Audio thread: Unable to lock mutex on DAWEngine");
                locked_daw.callback(data);
            },
            move |err| eprintln!("Audio error: {}", err),
        )
        .expect("Cannot set up output stream");
    stream.play().unwrap();
    {
        daw.lock().unwrap().add_pattern(Pattern::new(8, 64));
    }



    // UI
    let mut font = load_font_from_hex(File::open("font.hex").expect("font.hex not found! Please put an 8x8 font in HEX format and rename it to font.hex"));
    let ui_symbols = load_font_from_itf(File::open("font.itf").expect("font.itf not found! Please put an 8x8 font in ITF format and rename it to font.itf"));
    for i in 128..=201 { // Impulse Tracker symbols region
        // Replace the arrows plane
        font[8464+i] = ui_symbols[i];
    }

    let mut ui = TextUI::new(WIDTH, HEIGHT, font, "Project Corrosion");

    let ui_channel = ui.canvas.get_sender();

    // Title screen
    {
        #[cfg(not(debug_assertions))]
        let lines = 15usize;
        #[cfg(debug_assertions)]
        let lines = 19usize;
        let start_x = (WIDTH/16)-(" There may be bugs, glitches, crashes, and other unexpected issues. ".len()/2);
        let start_y = (HEIGHT/16)-(lines/2);

        let end_x = start_x+" There may be bugs, glitches, crashes, and other unexpected issues. ".len();
        let end_y = start_y+lines;

        fill_region(&ui_channel, Position { x: start_x-1, y: start_y-1 }, Position { x: end_x+1, y: end_y+1 }, MAIN_COLOR);
        fill_region(&ui_channel, Position { x: start_x, y: start_y+6 }, Position { x: end_x, y: start_y+11 }, 0xffff00);

        #[cfg(debug_assertions)]
        fill_region(&ui_channel, Position { x: start_x, y: start_y+12 }, Position { x: end_x, y: start_y+15 }, 0xff3f3f);

        ui_channel.send(Command::Text(start_x, start_y, 0, MAIN_COLOR, "Project Corrosion\nv0.0.0\n(C) Polyzium Productions 2023. All Rights Reserved.".to_string()));
        ui_channel.send(Command::Text(start_x, start_y+4, 0, MAIN_COLOR, format!(
            "Using {}, {} Hz, {} samples",
            host.id().name(),
            samplerate,
            match config.buffer_size {
                cpal::BufferSize::Default => "default number of".to_string(),
                Fixed(smp) => format!("{smp}"),
            })));
        ui_channel.send(Command::Text(start_x, (start_y)+6, 0, 0xffff00, "\n This software is currently in the alpha stage of development. \n There may be bugs, glitches, crashes, and other unexpected issues.\n Use at your own risk.\n".to_string()));
        #[cfg(debug_assertions)]
        ui_channel.send(Command::Text(start_x, (start_y)+11, 0, 0xff3f3f, "\n\n This is a development build. Expect a major performance impact. \n".to_string()));

        ui.widgets.push(Box::new(Button::new(
            Position { x: ((start_x+end_x)/2)-(" Continue ".len()/2), y: end_y-2 },
            " Continue ".to_string(),
            (0, MAIN_COLOR),
            (RIM_LIGHT, RIM_DARK)
        )));
    }

    let (events_tx, events_rx): (Sender<ui::events::Event>, Receiver<ui::events::Event>) = channel();
    ui.widgets.push(Box::new(EventHook {
        tx: events_tx,
        enabled: true,
    }));

    let mut enter_pressed = false;

    // Wait for the user to press continue
    while !(ui.wants_to_quit() || ui.widgets[0].clicked() || enter_pressed) {
        // ... or to press the enter key
        loop {
            match events_rx.try_recv() {
                Ok(e) => {
                    match e {
                        ui::events::Event::KeyDown(key) => {
                            match key {
                                Keycode::Return => {
                                    enter_pressed = true;
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                Err(_) => break,
            }
        }

        ui.poll();
        ui.update();
    }
    #[cfg(debug_assertions)]
    println!("User pressed continue");
    ui.widgets.clear();



    // Main application
    fill_region(&ui_channel, Position { x: 0, y: 0 }, Position { x: WIDTH/8, y: HEIGHT/8 }, MAIN_COLOR);
    let test = Box::new(LabelRuler {
        label: "Label Ruler Test".to_string(),
        bg_color: MAIN_COLOR,
        ruler_color: RIM_DARK,
        label_color: 0,
        start_x: 1,
        end_x: (WIDTH/8)-1,
        y: 3,
    });

    let mut pager = PagerContainer {
        widgets: Vec::with_capacity(16),
        current_widget: 0,
        handles_events: true,
    };
    let mut pattern_view = Box::new(PatternEditor::new(
        Position { x: 5, y: 6 }, Position { x: (WIDTH/8)-1, y: (HEIGHT/8)-1 }
    ));
    pattern_view.outer_bg = MAIN_COLOR;
    pattern_view.text_color = 0xffffff;
    pattern_view.top_rim = RIM_DARK;
    pattern_view.bottom_rim = RIM_LIGHT;
    pattern_view.key_mapping = default_kbd_mapping();
    // pview.pattern = Some(pat);

    let pattern_ruler = Box::new(LabelRuler {
        label: "Pattern Editor".to_string(),
        bg_color: MAIN_COLOR,
        ruler_color: RIM_DARK,
        label_color: 0,
        start_x: 1,
        end_x: (WIDTH/8)-1,
        y: 3,
    });

    let pattern_container = Box::new(Container {
        widgets: vec![pattern_ruler, pattern_view],
        handles_events: true,
    });

    let menu = Box::new(main_menu());
    let clock = Box::new(Clock {
        pos: Position { x: 0, y: 2 },
        bg_color: 0,
        color: 0xffffff,
        beat_color: 0,
        downbeat_color: 0,
        ticks: 0,
        ppq: daw.lock().unwrap().project.ppq,
        flash_on_beat: false,
        playing: false,
    });

    pager.widgets.push(test);
    pager.widgets.push(pattern_container);
    ui.widgets.push(Box::new(pager));
    ui.widgets.push(menu);
    ui.widgets.push(clock);

    let (events_tx, events_rx): (Sender<ui::events::Event>, Receiver<ui::events::Event>) = channel();
    ui.widgets.push(Box::new(EventHook {
        tx: events_tx,
        enabled: true,
    }));

    fill_region(&ui_channel, Position { x: 0, y: 0 }, Position { x: WIDTH/8, y: HEIGHT/8 }, MAIN_COLOR);
    ui_channel.send(Command::Text((WIDTH/16)-(54/2), 1, 0, MAIN_COLOR, "Project Corrosion v0.0.0 (C) 2023 Polyzium Productions".to_string()));

    #[cfg(debug_assertions)]
    ui_channel.send(Command::Text(0, 0, 0xffffff, 0xff0000, " DEBUG BUILD. Not for daily use. ".to_string()));

    // Main UI loop
    while !ui.wants_to_quit() {
        let mut locked_daw = daw.lock().expect("UI thread: Unable to lock mutex on DAWEngine");

        // Pattern Editor
        {
            let pager = get_widget_mut!(ui.widgets[0], PagerContainer);
            let container = get_widget_mut!(pager.widgets[1], Container);
            let mut patview = get_widget_mut!(container.widgets[1], PatternEditor);

            if patview.changed() {
                locked_daw.project.patterns[0] = patview.pattern.as_ref().unwrap().clone();
            }

            patview.pattern = Some(locked_daw.project.patterns[0].clone());
            patview.state = Some(locked_daw.state.patterns[0].clone());

            /* if !locked_daw.state.playing {
                patview.state.as_mut().unwrap().playing = false;
            } */
        }

        // Clock
        {
            let clock = get_widget_mut!(ui.widgets[2], Clock);

            clock.playing = locked_daw.state.patterns[0].playing;
            clock.ticks = locked_daw.state.patterns[0].position;
            clock.ppq = locked_daw.project.ppq;
        }
        // We updated the widgets with necessary data, unlock the mutex
        mem::drop(locked_daw);

        // Global event handler
        loop {
            match events_rx.try_recv() {
                Ok(e) => {
                    match e {
                        ui::events::Event::KeyDown(key) => {
                            match key {
                                Keycode::F1
                                    | Keycode::F2
                                    | Keycode::F3
                                    | Keycode::F4
                                    | Keycode::F5
                                    | Keycode::F6
                                    | Keycode::F7
                                    | Keycode::F8
                                    | Keycode::F9
                                    | Keycode::F10
                                    | Keycode::F11
                                    | Keycode::F12
                                => {
                                    fill_region(&ui_channel, Position { x: 0, y: 2 }, Position { x: WIDTH/8, y: HEIGHT/8 }, MAIN_COLOR);
                                }
                                _ => {}
                            }

                            let pager = get_widget_mut!(ui.widgets[0], PagerContainer);
                            match key {
                                Keycode::F1 => pager.current_widget = 0,
                                Keycode::F2 => pager.current_widget = 1,
                                // Keycode::Fx => ...
                                _ => {}
                            }

                            match key {
                                Keycode::Escape => {
                                    let menu = get_widget_mut!(ui.widgets[1], Menu);
                                    menu.visible = !menu.visible;
                                    ui.widgets[0] // pager
                                    .set_handles_events(false);

                                    let menu = get_widget_mut!(ui.widgets[1], Menu);
                                    if !menu.visible {
                                        menu.close();
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                Err(_) => break,
            }
        }

        // Menu handler
        let menu = get_widget_mut!(ui.widgets[1], Menu);
        handle_menu!(menu.pages[init::MENU_MAIN][1], { // Main -> Playback
            menu.goto_page(init::MENU_PLAYBACK);
        });
        handle_menu!(menu.pages[init::MENU_MAIN][3], { // Main -> Quit
            break;
        });
        handle_menu!(menu.pages[init::MENU_PLAYBACK][0], { // Main -> Playback -> Play
            let mut locked_daw = daw.lock().expect("UI thread: Unable to lock mutex on DAWEngine");
            locked_daw.state.playing = true;
            menu.close();
        });
        handle_menu!(menu.pages[init::MENU_PLAYBACK][1], { // Main -> Playback -> Pause
            let mut locked_daw = daw.lock().expect("UI thread: Unable to lock mutex on DAWEngine");
            locked_daw.state.playing = false;
            menu.close();
        });
        handle_menu!(menu.pages[init::MENU_PLAYBACK][2], { // Main -> Playback -> Stop
            let mut locked_daw = daw.lock().expect("UI thread: Unable to lock mutex on DAWEngine");
            locked_daw.state.playing = false;
            for state in &mut locked_daw.state.patterns {
                state.playing = false;
                state.position = 0;
            }
            menu.close();
        });

        // When menu is closed, let the pager handle events
        if !menu.visible && !ui.widgets[0].handles_events() {
            ui.widgets[0] // pager
            .set_handles_events(true);
        }

        // Update the UI
        ui.poll();
        // Draw
        ui.update();
    }
}
