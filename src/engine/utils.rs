use super::{
    pattern::Pattern,
    playlist::{Clip, PatternClip},
    state::PatternState,
};

impl super::DAWEngine {
    pub fn add_pattern(&mut self, pat: Pattern) {
        let rpb = pat.rpb;

        let state = PatternState {
            // pattern_index: self.project.patterns.len() - 1,
            position: 0,
            playing: false,
            row: 0,
            ticks_passed: 0,
            row_length: self.project.ppq as u32 / rpb as u32,
            note_ids: vec![0; pat.rows[0].len()],
        };

        self.project.patterns.push(pat);
        self.state.patterns.push(state);
    }

    pub fn pattern_to_clip(&self, index: usize) -> Clip {
        let pat = &self.project.patterns[index];

        Clip::Pattern(PatternClip {
            pattern_index: index,
            begin: 0,
            end: ((self.project.ppq as usize/pat.rpb as usize)*pat.rows.len()) as u32,
            offset: 0,
            track: 0,
        })
    }
}
