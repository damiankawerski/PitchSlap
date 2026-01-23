use crate::dsp::traits::FilterModule;

#[derive(Clone, Copy, Debug)]
pub struct NoiseGate {
    threshold: f32,
    attack: f32,
    release: f32,
    envelope: f32,
}

impl NoiseGate {
    pub fn new(threshold: f32, attack_ms: f32, release_ms: f32, sample_rate: f32) -> Self {
        Self {
            threshold,
            attack: (-1.0 / (attack_ms * 0.001 * sample_rate)).exp(),
            release: (-1.0 / (release_ms * 0.001 * sample_rate)).exp(),
            envelope: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();
        
        // Envelope follower
        if abs_input > self.envelope {
            self.envelope = self.attack * self.envelope + (1.0 - self.attack) * abs_input;
        } else {
            self.envelope = self.release * self.envelope + (1.0 - self.release) * abs_input;
        }

        // Gate
        if self.envelope > self.threshold {
            input
        } else {
            let gain = (self.envelope / self.threshold).min(1.0);
            input * gain
        }
    }

    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}


impl FilterModule for NoiseGate {
    fn process(&mut self, input: f32) -> f32 {
        self.process(input)
    }

    fn reset(&mut self) {
        self.reset();
    }
}