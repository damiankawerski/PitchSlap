use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct PeakingFilter {
    sample_rate: f32,
    /// Center frequency in Hz
    freq: f32,
    /// Gain in dB (-24.0 to 24.0)
    gain: f32,
    /// Q factor controlling bandwidth (0.1-10.0)
    q: f32,
    /// Previous input sample (x[n-1])
    x1: f32,
    /// Two-sample-delayed input (x[n-2])
    x2: f32,
    /// Previous output sample (y[n-1])
    y1: f32,
    /// Two-sample-delayed output (y[n-2])
    y2: f32,
}

impl PeakingFilter {
    /// Creates a new peaking filter.
    ///
    /// # Arguments
    ///
    /// * `freq` - Center frequency in Hz (minimum 1.0)
    /// * `gain` - Gain in dB (typically -24.0 to 24.0)
    /// * `q` - Q factor controlling bandwidth (0.1-10.0)
    ///
    /// # Returns
    ///
    /// A new `PeakingFilter` instance.
    pub fn new(sample_rate: f32, freq: f32, gain: f32, q: f32) -> Self {
        Self {
            sample_rate,
            freq: freq.max(1.0),
            gain,
            q: q.clamp(0.1, 10.0),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes a single sample through the peaking filter.
    ///
    /// # Arguments
    ///
    /// * `input` - Input audio sample
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    ///
    /// Filtered audio sample with peak boost or cut applied.
    pub fn process_internal(&mut self, input: f32, sample_rate: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * self.freq / sample_rate;
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let a = 10.0_f32.powf(self.gain / 40.0);
        let alpha = sin_omega / (2.0 * self.q);

        let b0 = 1.0 + (alpha * a);
        let b1 = -2.0 * cos_omega;
        let b2 = 1.0 - (alpha * a);
        let a0 = 1.0 + (alpha / a);
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - (alpha / a);

        let output = (b0 * input + b1 * self.x1 + b2 * self.x2 - a1 * self.y1 - a2 * self.y2) / a0;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Sets the center frequency.
    ///
    /// # Arguments
    ///
    /// * `freq` - Center frequency in Hz (minimum 1.0)
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.max(1.0);
    }

    /// Sets the gain.
    ///
    /// # Arguments
    ///
    /// * `gain` - Gain in dB (clamped to -24.0 to 24.0)
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(-24.0, 24.0);
    }

    /// Sets the Q factor.
    ///
    /// # Arguments
    ///
    /// * `q` - Q factor (clamped to 0.1-10.0)
    pub fn set_q(&mut self, q: f32) {
        self.q = q.clamp(0.1, 10.0);
    }

    /// Resets the filter state by clearing all delay samples.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl FilterModule for PeakingFilter {
    fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate;
        self.process_internal(input, sample_rate)
    }

    fn reset(&mut self) {
        self.reset();
    }
}
