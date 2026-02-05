use crate::dsp::traits::FilterModule;

/// Simple all-pass filter for reverb
#[derive(Debug, Clone)]
pub struct AllPassFilter {
    buffer: Vec<f32>,
    index: usize,
    gain: f32,
}

impl AllPassFilter {
    pub fn new(delay_samples: usize, gain: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples.max(1)],
            index: 0,
            gain,
        }
    }

    pub fn process_internal(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.index];
        let output = -self.gain * input + delayed;

        self.buffer[self.index] = input + self.gain * delayed;
        self.index = (self.index + 1) % self.buffer.len();

        output
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
    }
}

impl FilterModule for AllPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.process_internal(input)
    }

    fn reset(&mut self) {
        self.reset();
    }
}