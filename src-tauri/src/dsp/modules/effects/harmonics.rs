use crate::dsp::modules::utils::effect_parameter::EffectParameter;
use crate::dsp::modules::utils::frequency_tracker::FrequencyTracker;
use crate::dsp::modules::utils::oscilator::Oscillator;
use crate::dsp::traits::EffectModule;

#[derive(Debug, Clone)]
pub struct Harmonics {
    name: String,
    sample_rate: usize,
    harmonics_count: EffectParameter,
    intensity: EffectParameter,
    decay_rate: EffectParameter,
    oscillators: Vec<Oscillator>,
    fundamental_tracker: FrequencyTracker,
}

impl Harmonics {
    /// Creates a new harmonics effect for enriching vocal timbre.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Effect parameters including harmonics count, intensity, and decay rate
    ///
    /// # Returns
    ///
    /// A new `HarmonicsEffect` instance with initialized oscillators for harmonic generation.
    pub fn new(
        sample_rate: usize,
        harmonics_count: f32,
        intensity: f32,
        decay_rate: f32,
    ) -> Self {
        let mut effect = Self {
            name: "harmonics".to_string(),
            harmonics_count: EffectParameter::new("harmonics_count", harmonics_count, 1.0, 20.0),
            intensity: EffectParameter::new("intensity", intensity, 0.0, 1.0),
            decay_rate: EffectParameter::new("decay_rate", decay_rate, 0.0, 1.0),
            oscillators: Vec::new(),
            fundamental_tracker: FrequencyTracker::new(),
            sample_rate,
        };

        effect.initialize_oscillators();
        effect
    }

    pub fn process_internal(&mut self, input: &[f32], output: &mut [f32], sample_rate: f32) {
        let intensity = self.intensity.value;
        let decay_rate = self.decay_rate.value;

        for (i, sample) in input.iter().enumerate() {
            let fundamental = self.fundamental_tracker.process(*sample, sample_rate);

            if fundamental > 20.0 && fundamental < 2000.0 {
                let mut harmonics_sum = 0.0;

                for (i, oscillator) in self.oscillators.iter_mut().enumerate() {
                    let harmonic_freq = fundamental * (i + 2) as f32; // Start from 2nd harmonic
                    let harmonic_amp = intensity * decay_rate.powi(i as i32);

                    oscillator.set_frequency(harmonic_freq);
                    oscillator.set_amplitude(harmonic_amp);
                    harmonics_sum += oscillator.process(sample_rate);
                }

                output[i] += harmonics_sum;
            }
        }
    }

    fn initialize_oscillators(&mut self) {
        let count = self.harmonics_count.value as usize;
        self.oscillators.clear();

        for i in 1..=count {
            self.oscillators
                .push(Oscillator::new(440.0 * i as f32, 0.1 / i as f32));
        }
    }
}

impl EffectModule for Harmonics {
    fn name(&self) -> &str {
        &self.name
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        self.process_internal(input, output, self.sample_rate as f32);
    }

    fn reset(&mut self) {
        for oscillator in &mut self.oscillators {
            oscillator.reset();
        }
        self.fundamental_tracker.reset();
    }
}
