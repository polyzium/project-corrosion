use super::{
    pattern::Pattern,
    playlist::{Clip, PatternClip},
    state::PatternState,
};

impl super::DAWEngine {
    pub fn add_pattern(&mut self, pat: Pattern) {
        let rpb = pat.rpb;

        self.project.patterns.push(pat);
        self.state.patterns.push(PatternState {
            pattern_index: self.project.patterns.len() - 1,
            position: 0,
            playing: false,
            row: 0,
            ticks_passed: 0,
            row_length: self.project.ppq as u32 / rpb as u32,
        })
    }

    pub fn pattern_to_clip(&self, index: usize) -> Clip {
        let pat = &self.project.patterns[index];

        Clip::Pattern(PatternClip {
            pattern_index: index,
            begin: 0,
            end: (((pat.rows.len() as f32 / pat.rpb as f32) * 4.0) as u32) - 1,
            offset: 0,
            track: 0,
        })
    }
}
