use super::{playlist::{PatternClip, Clip}, pattern::Pattern, state::PatternState};

impl super::DAWEngine {
    pub fn add_pattern(&mut self, pat: Pattern) {
        self.project.patterns.push(pat);
        self.state.patterns.push(PatternState { pattern_index: self.project.patterns.len()-1, position: 0, playing: false, row: 0, ticks_passed: 0 } )
    }

    pub fn pattern_to_clip(&self, index: usize) -> Clip {
        let pat = &self.project.patterns[index];

        Clip::Pattern(PatternClip {
            pattern_index: index,
            begin: 0,
            end: if pat.rows.len() < (self.project.rpb as usize*4) {
				self.project.rpb as u32*self.project.ppq as u32*4
			} else {
				(pat.rows.len() * self.project.ppq as usize/self.project.rpb as usize) as u32
			},
            offset: 0,
            track: 0,
        })
    }
}