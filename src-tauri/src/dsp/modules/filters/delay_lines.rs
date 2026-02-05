use std::collections::VecDeque;
use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct DelayLine {
    /// Circular buffer storing delayed samples
    buffer: VecDeque<f32>,
    /// Maximum delay capacity in samples
    max_delay_samples: usize,
    /// Current delay amount in samples (supports fractional values)
    delay_samples: f32,
    /// Feedback amount (0.0-0.99)
    feedback: f32,
}

impl DelayLine {
    /// Creates a new delay line with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `max_delay_samples` - Maximum delay capacity in samples
    /// * `delay_samples` - Initial delay amount in samples
    /// * `feedback` - Feedback amount (0.0-0.99)
    ///
    /// # Returns
    ///
    /// A new `DelayLine` instance initialized with zeros.
    pub fn new(max_delay_samples: usize, delay_samples: f32, feedback: f32) -> Self {
        let mut buffer = VecDeque::with_capacity(max_delay_samples);
        buffer.resize(max_delay_samples, 0.0);

        Self {
            buffer,
            max_delay_samples,
            delay_samples: delay_samples.clamp(0.0, max_delay_samples as f32),
            feedback: feedback.clamp(0.0, 0.99),
        }
    }

    /// Processes a single sample through the delay line.
    ///
    /// Uses linear interpolation for fractional delays and applies feedback.
    ///
    /// # Arguments
    ///
    /// * `input` - Input audio sample
    ///
    /// # Returns
    ///
    /// Delayed and possibly feedback-processed audio sample.
    pub fn process_internal(&mut self, input: f32) -> f32 {
        // Simple linear interpolation for fractional delay
        let delay_int = self.delay_samples.floor() as usize;
        let delay_frac = self.delay_samples - delay_int as f32;

        let sample1 = self.buffer.get(delay_int).copied().unwrap_or(0.0);
        let sample2 = self.buffer.get(delay_int + 1).copied().unwrap_or(0.0);

        let delayed_sample = sample1 * (1.0 - delay_frac) + sample2 * delay_frac;

        // Add input with feedback
        let output = input + delayed_sample * self.feedback;

        // Push new sample to buffer
        self.buffer.pop_front();
        self.buffer.push_back(output);

        delayed_sample
    }

    /// Sets the delay amount in samples.
    ///
    /// # Arguments
    ///
    /// * `delay_samples` - New delay amount (clamped to 0.0 to max_delay_samples)
    pub fn set_delay(&mut self, delay_samples: f32) {
        self.delay_samples = delay_samples.clamp(0.0, self.max_delay_samples as f32);
    }

    /// Sets the feedback amount.
    ///
    /// # Arguments
    ///
    /// * `feedback` - New feedback amount (clamped to 0.0-0.99 for stability)
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.99);
    }

    /// Clears the delay buffer by setting all samples to zero.
    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0.0);
    }
}

impl FilterModule for DelayLine {
    fn process(&mut self, input: f32) -> f32 {
        self.process_internal(input)
    }

    fn reset(&mut self) {
        self.clear();
    }
}