#[derive(Debug, Clone)]
pub struct LFO {
    /// LFO frequency in Hz
    frequency: f32,
    /// Modulation depth/amplitude (0.0-1.0)
    amplitude: f32,
    /// Current phase position (0.0-1.0)
    phase: f32,
    /// Waveform shape
    waveform: LFOWaveform,
    /// Sample rate in Hz
    sample_rate: f32,
}

impl LFO {
    /// Creates a new LFO with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `frequency` - LFO frequency in Hz (minimum 0.1)
    /// * `amplitude` - Modulation depth (0.0-1.0)
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Returns
    ///
    /// A new `LFO` instance with sine waveform by default.
    pub fn new(frequency: f32, amplitude: f32, sample_rate: f32) -> Self {
        Self {
            frequency: frequency.max(0.1),
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: 0.0,
            waveform: LFOWaveform::Sine,
            sample_rate: sample_rate.max(1.0),
        }
    }

    /// Generates the next LFO output sample.
    ///
    /// # Returns
    ///
    /// Modulation value in range -amplitude to +amplitude.
    pub fn process(&mut self) -> f32 {
        let output = match self.waveform {
            LFOWaveform::Sine => (self.phase * 2.0 * std::f32::consts::PI).sin(),
            LFOWaveform::Triangle => {
                let normalized = self.phase.fract();
                if normalized < 0.5 {
                    4.0 * normalized - 1.0
                } else {
                    3.0 - 4.0 * normalized
                }
            }
            LFOWaveform::Sawtooth => 2.0 * self.phase.fract() - 1.0,
            LFOWaveform::Square => {
                if self.phase.fract() < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            LFOWaveform::Random => {
                // Simple pseudo-random using linear congruential generator
                let mut state = (self.phase * 1000.0) as u32;
                state = state.wrapping_mul(1103515245).wrapping_add(12345);
                (state as f32 / u32::MAX as f32) * 2.0 - 1.0
            }
        };

        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        output * self.amplitude
    }

    /// Sets the LFO frequency.
    ///
    /// # Arguments
    ///
    /// * `frequency` - New frequency in Hz (minimum 0.1)
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.max(0.1);
    }

    /// Sets the modulation amplitude/depth.
    ///
    /// # Arguments
    ///
    /// * `amplitude` - New amplitude (clamped to 0.0-1.0)
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude.clamp(0.0, 1.0);
    }

    /// Sets the waveform shape.
    ///
    /// # Arguments
    ///
    /// * `waveform` - New waveform type
    pub fn set_waveform(&mut self, waveform: LFOWaveform) {
        self.waveform = waveform;
    }

    /// Resets the LFO phase to zero.
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LFOWaveform {
    /// Smooth sinusoidal waveform
    Sine,
    /// Linear ramp up and down triangle waveform
    Triangle,
    /// Linear ramp sawtooth waveform
    Sawtooth,
    /// Instant-transition square waveform
    Square,
    /// Pseudo-random noise waveform
    Random,
}

