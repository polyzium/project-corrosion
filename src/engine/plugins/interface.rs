use std::{sync::mpsc};

// use super::builtin::midi::plugin::MidiOutPlugin;

// Describes a parameter
pub struct Parameter {
    pub index: usize,
    pub name: String,
    pub value: u8, // Max is 127 per MIDI standard
    pub min: u8,
    pub max: u8,
}

pub struct TimedEvent {
    pub module_index: usize,

    pub position: u32, // position in samples relative to start of the buffer
    pub event: Event
}

pub enum Event {
    NoteOff{id: usize, key: u8, vel: u8},
    NoteOn{id: usize, key: u8, vel: u8},
    ControlChange{index: u8, value: u8},

    // polyphonic expressions (for MPE and CLAP)
    ExprPitch{id: usize, target_pitch: f32}, // target pitch in semitones relative to the currently playing note
    ExprVolume{id: usize, target_vol: u8} // 0..=127
}

#[derive(Clone, Copy)]
pub struct NoteState {
    pub id: usize,
    pub instrument: usize,

    pub key: u8,
    pub vel: u8,
    pub pitch_bend: f32, // in semitones, relative to current note
    pub is_on: bool,
}

#[derive(Debug)]
pub enum PluginError {
    NoSuchPlugin, // Builtins only, there isn't such a plugin
    LoadError(String), // Failed to load an external (VST/CLAP/etc) plugin
    InitError(String) // Plugin loaded, but failed to initialize
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::NoSuchPlugin => write!(f, "No such plugin"),
            PluginError::LoadError(desc) => write!(f, "Unable to load plugin: {desc}"),
            PluginError::InitError(desc) => write!(f, "Unable to initialize plugin: {desc}"),
        }
    }
}

pub trait Plugin {
    fn new(path: &str) -> Result<Self, PluginError> where Self: Sized;
    fn process(&mut self, events: &[TimedEvent], input: &[f32], output: &mut [f32]);
    // fn note_states(&self) -> &[NoteState];
    fn show_gui(&mut self, shown: bool);
    fn get_params(&self) -> Vec<Parameter>;

    fn enable(&mut self);
    fn disable(&mut self);
    fn active(&self) -> bool;
}