use std::f32::consts::TAU;

pub struct GoertzelSine {
    coeff: f32,
    a: f32,
    b: f32
}

impl GoertzelSine {
    pub fn new(freq: f32, phase: f32, samplerate: u32) -> GoertzelSine {
        let normf = freq/samplerate as f32;
        GoertzelSine { coeff: 2.0*(normf*TAU).cos(), a: phase.cos(), b: (normf*TAU + phase).cos() }
    }

    pub fn process(&mut self) -> f32 {
        let tmp = self.b;

        self.b = self.b*self.coeff - self.a;
        self.a = tmp;

        return self.b;
    }
}