use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct LowPassFilter {
    /// Sample rate in Hz (used for cutoff frequency calculations)
    sample_rate: f32,
    /// Cutoff frequency in Hz
    cutoff: f32,
    /// Resonance/Q factor (0.1-10.0)
    resonance: f32,
    /// Previous input sample (x[n-1])
    x1: f32,
    /// Two-sample-delayed input (x[n-2])
    x2: f32,
    /// Previous output sample (y[n-1])
    y1: f32,
    /// Two-sample-delayed output (y[n-2])
    y2: f32,
}

impl LowPassFilter {
    /// Creates a new low-pass filter.
    ///
    /// # Arguments
    ///
    /// * `cutoff` - Cutoff frequency in Hz (minimum 1.0)
    /// * `resonance` - Q factor controlling resonance (0.1-10.0)
    ///
    /// # Returns
    ///
    /// A new `LowPassFilter` instance.
    pub fn new(sample_rate: f32, cutoff: f32, resonance: f32) -> Self {
        Self {
            sample_rate,
            cutoff: cutoff.max(1.0),
            resonance: resonance.clamp(0.1, 10.0),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes a single sample through the low-pass filter.
    ///
    /// # Arguments
    ///
    /// * `input` - Input audio sample
    ///
    /// # Returns
    ///
    /// Filtered audio sample.
    pub fn process_internal(&mut self, input: f32) -> f32 {
        // Simple 2-pole Butterworth filter
        let omega = 2.0 * std::f32::consts::PI * self.cutoff / self.sample_rate;
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let q = self.resonance;

        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        let output = (b0 * input + b1 * self.x1 + b2 * self.x2 - a1 * self.y1 - a2 * self.y2) / a0;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Sets the cutoff frequency.
    ///
    /// # Arguments
    ///
    /// * `cutoff` - Cutoff frequency in Hz (minimum 1.0)
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(1.0);
    }

    /// Sets the resonance/Q factor.
    ///
    /// # Arguments
    ///
    /// * `resonance` - Q factor (clamped to 0.1-10.0)
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.1, 10.0);
    }

    /// Resets the filter state by clearing all delay samples.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl FilterModule for LowPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.process_internal(input)
    }

    fn reset(&mut self) {
        self.reset();
    }
}
