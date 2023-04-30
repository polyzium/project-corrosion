const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const NOTE_AFTERTOUCH: u8 = 0xA0;
const CONTROL_CHANGE: u8 = 0xB0;
const PROGRAM_CHANGE: u8 = 0xC0;
const CHANNEL_AFTERTOUCH: u8 = 0xD0;
const PITCH_BEND: u8 = 0xE0;

use midir::{MidiOutputPort, MidiOutputConnection};
use crate::engine::{plugins::interface::{Plugin, Event, Parameter, PluginError, TimedEvent, NoteState}};

pub struct MidiOutPlugin {
    pub mpe_enabled: bool,
    pub mpe_bend_range: u8,

    ports: Vec<MidiOutputPort>,
    state: Vec<NoteState>,
    midi_conn: midir::MidiOutputConnection,
}

impl MidiOutPlugin {
    pub fn get_ports(&self) -> &[MidiOutputPort] {
        &self.ports
    }

    pub fn change_port(&mut self, index: usize) {
        // TODO error handling
        let midi_out = midir::MidiOutput::new("Project Corrosion MIDI Out").unwrap();
        let ports = midi_out.ports();
        self.midi_conn = midi_out.connect(&ports[index], "corrosion-midi").unwrap();
        self.ports = ports;
    }
}

impl Plugin for MidiOutPlugin {
    fn new(path: &str) -> Result<MidiOutPlugin, PluginError> {
        let midi_out = match midir::MidiOutput::new("Project Corrosion MIDI Out") {
            Ok(output) => output,
            Err(err) => { return Err(PluginError::InitError(format!("{}", err))) },
        };

        let ports = midi_out.ports();
        let midi_conn: MidiOutputConnection;
        if ports.len() != 0 {
            midi_conn = match midi_out.connect(&ports[0], "corrosion-midi") {
                Ok(conn) => conn,
                Err(err) => { return Err(PluginError::InitError(format!("{}", err))) },
            };
        } else {
            return Err(PluginError::InitError("There are no available MIDI ports".to_string()))
        };

        Ok(MidiOutPlugin {
            mpe_enabled: true,
            mpe_bend_range: 24, // two octaves
            ports,
            state: Vec::with_capacity(16),
            midi_conn,
        })
    }

    fn process(&mut self, events: &[TimedEvent], _input: &[f32], _output: &mut [f32]) {
        for e in events {
            if self.mpe_enabled {
                // Note ID = channel ID, because MPE has 1 note per channel.
                // MIDI has 16 channels per port, so 16 note expressive polyphony is guaranteed.
                match e.event {
                    Event::NoteOff { id, key, vel } => { self.midi_conn.send(&[NOTE_OFF | id as u8, key, vel]); },
                    Event::NoteOn { id, key, vel } => { self.midi_conn.send(&[NOTE_ON | id as u8, key, vel]); },
                    Event::ControlChange {..} => { println!("MidiOutPlugin: TODO: received ControlChange, but don't know to which channel to send!") }, // TODO figure this out. Probably all channels?
                    Event::ExprPitch { id, target_pitch } => {
                        let midi_pitch = (target_pitch) as u16;
                        let u14 = (midi_pitch << 2) & 0x3FFF;

                        let msb = (u14 >> 7) as u8;
                        let lsb = (u14 & 0x7F) as u8;

                        self.midi_conn.send(&[PITCH_BEND | id as u8, msb, lsb]);
                    },
                    // CC #7 is volume
                    Event::ExprVolume { id, target_vol } => { self.midi_conn.send(&[CONTROL_CHANGE | id as u8, 7, target_vol]); },
                }
            } else {
                todo!("Non-MPE MIDI is not implemented yet.")
            }
        }
    }

    /* fn note_states(&self) -> &[NoteState] {
        &self.state
    } */

    fn show_gui(&mut self, shown: bool) {
        todo!()
    }

    fn get_params(&self) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = Vec::with_capacity(119); // a MIDI channel has 119 CCs

        for cc in 0..=119 {
            params.push(Parameter {
                index: cc,
                name: format!("Continuous Controller #{cc}"),
                value: 255, // we cannot get MIDI CCs in real time, we can only remember them. TODO I guess?
                min: 0,
                max: 127,
            });
        }

        params
    }

    fn enable(&mut self) {
        // no-op
    }

    fn disable(&mut self) {
        // no-op
    }

    fn active(&self) -> bool {
        return true
    }
}