use std::sync::mpsc;

use clap_sys::{plugin::clap_plugin, entry::{clap_plugin_entry, self}};
use libloading::{Library, Symbol};

use super::interface::{Plugin, PluginError, Event, Parameter};

struct ClapPlugin {
    active: bool,
    lib: Library
}

macro_rules! plugin_load_error {
    ($err:ident) => {
        return Err(PluginError::LoadError(format!("{}", $err)))
    };
}

impl Plugin for ClapPlugin {
    fn new(path: &str) -> Result<Self, PluginError> {
        let lib: libloading::Library;
        let clap_entry: Symbol<clap_plugin_entry>;

        unsafe {
            match libloading::Library::new(path) {
                Ok(loaded_lib) => lib = loaded_lib,
                Err(err) => plugin_load_error!(err)
            }

            /* match lib.get(b"clap_entry") {
                Ok(entry) => clap_entry = *entry,
                Err(err) => return Err(plugin_load_error!(err)),
            } */
        }

        let mut this = ClapPlugin { lib, active: false };
        unsafe {
            match this.lib.get::<clap_plugin_entry>(b"clap_entry") {
                Ok(symbol) => clap_entry = symbol,
                Err(err) => plugin_load_error!(err),
            }
        }

        // OK. Now what?

        todo!()
    }

    fn process(&mut self, events: &[super::interface::TimedEvent], input: &[f32], output: &mut [f32]) {
        todo!()
    }

    /* fn note_states(&self) -> &[super::interface::NoteState] {
        todo!()
    } */

    fn show_gui(&mut self, shown: bool) {
        todo!()
    }

    fn get_params(&self) -> Vec<Parameter> {
        todo!()
    }

    fn enable(&mut self) {
        todo!()
    }

    fn disable(&mut self) {
        todo!()
    }

    fn active(&self) -> bool {
        self.active
    }
}