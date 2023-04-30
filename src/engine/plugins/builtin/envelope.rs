// Ye Olde ADSR envelope

// TODO fix stage times, so they are in seconds
// As of now, decay_time of 1.0 takes 9.210182 seconds to decay from 1 to 0

/* const T60: f32 = 10.0f32.powf(-60.0/20.0);
const T80: f32 = 10.0f32.powf(-80.0/20.0); */

pub struct Adsr {
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    sample_rate: f32,
    smoothing_factor: f32,
    state: EnvelopeState,
    value: f32,
}

impl Adsr {
    pub fn new(
        attack_time: f32,
        decay_time: f32,
        sustain_level: f32,
        release_time: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            attack_time,
            decay_time,
            sustain_level,
            release_time,
            sample_rate,
            state: EnvelopeState::Idle,
            value: 0.0,
            smoothing_factor: 1.0,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.smoothing_factor = (1.0 / self.attack_time) / self.sample_rate;
    }

    pub fn release(&mut self) {
        self.state = EnvelopeState::Release;
        self.smoothing_factor = (1.0 / self.release_time) / self.sample_rate;
    }

    pub fn is_idle(&self) -> bool {
        self.state == EnvelopeState::Idle
    }

    pub fn process(&mut self) -> f32 {
        match self.state {
            EnvelopeState::Idle => {
                self.value = 0.0;
            }
            EnvelopeState::Attack => {
                self.value += (1.0 - self.value) * self.smoothing_factor;
                if self.value >= 0.999 {
                    self.value = 1.0;
                    self.state = EnvelopeState::Decay;
                    self.smoothing_factor = (1.0 / self.decay_time) / self.sample_rate;
                }
            }
            EnvelopeState::Decay => {
                self.value += (self.sustain_level - self.value) * self.smoothing_factor;
                if self.value <= self.sustain_level {
                    self.value = self.sustain_level;
                    self.state = EnvelopeState::Sustain;
                }
            }
            EnvelopeState::Sustain => {
                self.value = self.sustain_level;
            }
            EnvelopeState::Release => {
                self.value += (0.0 - self.value) * self.smoothing_factor;
                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.state = EnvelopeState::Idle;
                }
            }
        }
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

// "Classic" envelope, exists there solely for compatibility with old tracker files
struct ClassicEnvelopePoint {
    tick: u8,
    level: f32,
}

struct ClassicEnvelope {
    pub points: Vec<ClassicEnvelopePoint>,
    pub env_loop: (usize, usize),
    pub env_sustain: (usize, usize),
    pub env_loop_enabled: bool,
    pub env_sustain_enabled: bool,

    pub tickrate: u8,
    pub tempo: u8,
    pub samplerate: u32,

    samples_passed: u16,
    current_tick: u8,
    tick_length: u16,

    triggered: bool,
    playing: bool,
}

impl ClassicEnvelope {
    pub fn new(tickrate: u8, tempo: u8, samplerate: u32) -> Self {
        let tick_length = ((60.0 / tempo as f32) * samplerate as f32) as u16;

        Self {
            points: Vec::new(),
            env_loop: (0, 0), // Envelope loop begin and end
            env_sustain: (0, 0), // Sustain loop begin and end
            tickrate,
            tempo,
            samplerate,
            samples_passed: 0,
            current_tick: 0,
            tick_length,
            triggered: false,
            playing: false,
            env_loop_enabled: false,
            env_sustain_enabled: false,
        }
    }

    pub fn change_tempo(&mut self, tempo: u8) {
        self.tempo = tempo;
        self.tick_length = ((60.0 / tempo as f32) * self.samplerate as f32) as u16;
    }

    pub fn process(&mut self) -> f32 {
        if !self.playing {
            return self.points[0].level;
        }

        if self.samples_passed >= self.tick_length {
            self.current_tick += 1;

            // Stop envelope when it's at the last point
            if self.current_tick > self.points[self.points.len()-1].tick {
                self.playing = false;
                self.current_tick = 0;
            }
            
            if self.env_sustain_enabled && self.triggered {
                // Jump to the beginning of the loop
                if self.current_tick > self.points[self.env_sustain.1].tick {
                    self.current_tick = self.points[self.env_sustain.0].tick
                }
            } else if self.env_loop_enabled {
                // Jump to the beginning of the loop
                if self.current_tick > self.points[self.env_loop.1].tick {
                    self.current_tick = self.points[self.env_loop.0].tick
                }
            }
        }
        self.value(self.current_tick)
    }

    fn value(&self, tick: u8) -> f32 {
        if tick <= self.points[0].tick {
            return self.points[0].level;
        }

        for i in 1..self.points.len() {
            let prev_point = &self.points[i - 1];
            let next_point = &self.points[i];

            if tick < next_point.tick {
                let tick_diff = next_point.tick - prev_point.tick;
                let level_diff = next_point.level - prev_point.level;
                let tick_ratio = (tick - prev_point.tick) as f32 / tick_diff as f32;

                return prev_point.level + (level_diff * tick_ratio);
            }
        }

        return self.points[self.points.len() - 1].level;
    }

    fn trigger(&mut self) {
        self.playing = true;
        self.triggered = true;
    }

    fn release(&mut self) {
        self.triggered = false;
    }
}
