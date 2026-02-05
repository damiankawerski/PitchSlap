#[derive(Debug, Clone)]
pub struct FrequencyTracker {
    buffer: Vec<f32>,
    buffer_index: usize,
}

impl FrequencyTracker {
    pub fn new() -> Self {
        Self {
            buffer: vec![0.0; 1024],
            buffer_index: 0,
        }
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        self.buffer[self.buffer_index] = input;
        self.buffer_index = (self.buffer_index + 1) % self.buffer.len();

        // Simple autocorrelation-based pitch detection
        let mut best_period = 0;
        let mut best_correlation = 0.0;

        let min_period = (sample_rate / 2000.0) as usize; // 2000 Hz max
        let max_period = (sample_rate / 50.0) as usize; // 50 Hz min

        for period in min_period..max_period.min(self.buffer.len() / 2) {
            let mut correlation = 0.0;
            for i in 0..self.buffer.len() - period {
                correlation += self.buffer[i] * self.buffer[i + period];
            }

            if correlation > best_correlation {
                best_correlation = correlation;
                best_period = period;
            }
        }

        if best_period > 0 && best_correlation > 0.1 {
            sample_rate / best_period as f32
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.buffer_index = 0;
    }
}