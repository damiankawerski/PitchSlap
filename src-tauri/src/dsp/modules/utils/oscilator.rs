#[derive(Debug, Clone)]
pub struct Oscillator {
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

impl Oscillator {
    pub fn new(frequency: f32, amplitude: f32) -> Self {
        Self {
            frequency: frequency.max(1.0),
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: 0.0,
        }
    }

    pub fn process(&mut self, sample_rate: f32) -> f32 {
        let output = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude;
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        output
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.max(1.0);
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude.clamp(0.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}