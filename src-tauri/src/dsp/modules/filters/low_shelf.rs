use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct LowShelfFilter {
    sample_rate: f32,
    /// Shelf frequency in Hz
    freq: f32,
    /// Gain in dB (-24.0 to 24.0)
    gain: f32,
    /// Previous input sample (x[n-1])
    x1: f32,
    /// Previous output sample (y[n-1])
    y1: f32,
}

impl LowShelfFilter {
    /// Creates a new low shelf filter.
    ///
    /// # Arguments
    ///
    /// * `freq` - Shelf frequency in Hz (minimum 1.0)
    /// * `gain` - Gain in dB (typically -24.0 to 24.0)
    ///
    /// # Returns
    ///
    /// A new `LowShelfFilter` instance.
    pub fn new(sample_rate: f32, freq: f32, gain: f32) -> Self {
        Self {
            sample_rate,
            freq: freq.max(1.0),
            gain,
            x1: 0.0,
            y1: 0.0,
        }
    }

    /// Processes a single sample through the low shelf filter.
    ///
    /// # Arguments
    ///
    /// * `input` - Input audio sample
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    ///
    /// Filtered audio sample with low frequency gain applied.
    pub fn process_internal(&mut self, input: f32, sample_rate: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * self.freq / sample_rate;
        let s = omega.sin();
        let c = omega.cos();
        let a = 10.0_f32.powf(self.gain / 40.0);

        let beta = a.sqrt() / 1.0; // Q = 1

        let b0 = a * ((a + 1.0) - (a - 1.0) * c + beta * s);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * c);
        let a0 = (a + 1.0) + (a - 1.0) * c + beta * s;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * c);
        let a2 = (a + 1.0) + (a - 1.0) * c - beta * s;

        let output = (b0 * input + b1 * self.x1 - a1 * self.y1 - a2 * 0.0) / a0;

        self.x1 = input;
        self.y1 = output;

        output
    }

    /// Sets the shelf frequency.
    ///
    /// # Arguments
    ///
    /// * `freq` - Shelf frequency in Hz (minimum 1.0)
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

    /// Resets the filter state by clearing all delay samples.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}


impl FilterModule for LowShelfFilter {
    fn process(&mut self, input: f32) -> f32 {
        // Assuming a fixed sample rate for now; you might want to pass this differently
        let sample_rate = self.sample_rate;
        self.process_internal(input, sample_rate)
    }

    fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}