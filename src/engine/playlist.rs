// the u32 fields are units in ticks

struct Playlist {
	playhead: u32, // in ticks
	clips: Vec<Box<dyn Clip>>
}

trait Clip {
	fn pos_begin(&self) -> u32;
	fn pos_end(&self) -> u32;
	fn offset(&self) -> u32; // offset relative to the start
}

pub struct PatternClip<'a> {
	pub pattern: &'a super::pattern::Pattern,
	pub begin: u32,
	pub end: u32,
	pub offset: u32
}

impl Clip for PatternClip<'_> {
	fn pos_begin(&self) -> u32 { self.begin }
	fn pos_end(&self) -> u32 { self.end }
	fn offset(&self) -> u32 { self.offset }
}

/* struct AudioClip {
	
}

impl Clip for AudioClip {

} */