use super::playlist::Playlist;
use crate::engine::pattern::Pattern;

pub struct Project {
    pub ppq: u16, // pulses per quarter
    pub tempo: u16,

    pub playlist: Playlist,
    pub patterns: Vec<Pattern>,
}
