// all units are in ticks

pub struct Playlist {
    pub clips: Vec<Clip>,
}

#[allow(dead_code)]
pub enum Clip {
    Pattern(PatternClip), // Audio(AudioClip)
}

// TODO: Add validation to the PatternClip struct to ensure that the begin, end, and offset fields are all within the bounds of the pattern.
// This would prevent bugs caused by invalid values.
trait ClipInfo {
    fn pos_begin(&self) -> u32;
    fn pos_end(&self) -> u32;
    fn offset(&self) -> u32; // offset relative to pos_begin, will start playing clip's content at the position based on it
    fn track(&self) -> u8;
    fn length(&self) -> u32 {
        self.pos_end() - self.pos_begin()
    }
}

pub struct PatternClip {
    pub pattern_index: usize,
    pub begin: u32,
    pub end: u32,
    pub offset: u32,
    pub track: u8,
}

impl ClipInfo for PatternClip {
    fn pos_begin(&self) -> u32 {
        self.begin
    }
    fn pos_end(&self) -> u32 {
        self.end
    }
    fn offset(&self) -> u32 {
        self.offset
    }
    fn track(&self) -> u8 {
        self.track
    }
}

impl ClipInfo for Clip {
    fn pos_begin(&self) -> u32 {
        match self {
            Clip::Pattern(clip) => clip.pos_begin(),
        }
    }

    fn pos_end(&self) -> u32 {
        match self {
            Clip::Pattern(clip) => clip.pos_end(),
        }
    }

    fn offset(&self) -> u32 {
        match self {
            Clip::Pattern(clip) => clip.offset(),
        }
    }

    fn track(&self) -> u8 {
        match self {
            Clip::Pattern(clip) => clip.track(),
        }
    }
}

/* struct AudioClip {
    // TODO
}

impl Clip for AudioClip {

} */
