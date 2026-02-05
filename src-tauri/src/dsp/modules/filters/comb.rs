use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct CombFilter {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
    damping: f32,
    filter_store: f32,
}

impl CombFilter {
    pub fn new(delay_samples: usize, feedback: f32, damping: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples.max(1)],
            index: 0,
            feedback,
            damping,
            filter_store: 0.0,
        }
    }

    pub fn process_internal(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];

        // One-pole low-pass filter for damping
        self.filter_store = output * (1.0 - self.damping) + self.filter_store * self.damping;

        self.buffer[self.index] = input + self.filter_store * self.feedback;
        self.index = (self.index + 1) % self.buffer.len();

        output
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
        self.filter_store = 0.0;
    }
}

impl FilterModule for CombFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.process_internal(input)
    }

    fn reset(&mut self) {
        self.reset();
    }
}