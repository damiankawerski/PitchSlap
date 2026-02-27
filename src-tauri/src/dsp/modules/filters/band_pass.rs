use crate::dsp::traits::FilterModule;

#[derive(Debug, Clone)]
pub struct BandPassFilter {
    sample_rate: f32,
    /// Center frequency in Hz
    center_freq: f32,
    /// Bandwidth in Hz
    bandwidth: f32,
    /// Previous input sample (x[n-1])
    x1: f32,
    /// Two-sample-delayed input (x[n-2])
    x2: f32,
    /// Previous output sample (y[n-1])
    y1: f32,
    /// Two-sample-delayed output (y[n-2])
    y2: f32,
}

impl BandPassFilter {
    /// Creates a new band-pass filter.
    ///
    /// # Arguments
    ///
    /// * `center_freq` - Center frequency in Hz (minimum 1.0)
    /// * `bandwidth` - Bandwidth in Hz (minimum 1.0)
    ///
    /// # Returns
    ///
    /// A new `BandPassFilter` instance.
    pub fn new(sample_rate: f32, center_freq: f32, bandwidth: f32) -> Self {
        Self {
            sample_rate,
            center_freq: center_freq.max(1.0),
            bandwidth: bandwidth.max(1.0),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes a single sample through the band-pass filter.
    ///
    /// # Arguments
    ///
    /// * `input` - Input audio sample
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    ///
    /// Filtered audio sample.
    pub fn process_internal(&mut self, input: f32, sample_rate: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * self.center_freq / sample_rate;
        let q = self.center_freq / self.bandwidth;
        let alpha = omega.sin() / (2.0 * q);

        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha;

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
    pub fn set_center_freq(&mut self, freq: f32) {
        self.center_freq = freq.max(1.0);
    }

    /// Sets the bandwidth.
    ///
    /// # Arguments
    ///
    /// * `bandwidth` - Bandwidth in Hz (minimum 1.0)
    pub fn set_bandwidth(&mut self, bandwidth: f32) {
        self.bandwidth = bandwidth.max(1.0);
    }

    /// Resets the filter state by clearing all delay samples.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl FilterModule for BandPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let sample_rate = self.sample_rate;
        self.process_internal(input, sample_rate)
    }

    fn reset(&mut self) {
        self.reset();
    }
}
