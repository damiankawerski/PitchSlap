use crate::dsp::modules::filters::DelayLine;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;
use crate::dsp::modules::utils::lfo::LFO;
use crate::dsp::traits::EffectModule;

#[derive(Debug, Clone)]
pub struct Vibrato {
    name: String,
    intensity: EffectParameter,
    lfo: LFO,
    delay_line: DelayLine,
    sample_rate: f32,
}

impl Vibrato {
    /// Creates a new vibrato effect for pitch modulation.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Effect parameters including rate, depth, and intensity
    ///
    /// # Returns
    ///
    /// A new `Vibrato` instance with LFO-controlled delay line for pitch variation.
    pub fn new(rate: f32, depth: f32, intensity: f32, sample_rate: f32) -> Self {
        let effect = Self {
            name: "vibrato".to_string(),
            lfo: LFO::new(rate, depth, sample_rate),
            delay_line: DelayLine::new(1024, 512.0, 0.0), // No feedback for vibrato
            sample_rate,
            intensity: EffectParameter::new("intensity", intensity, 0.0, 1.0),
        };

        effect
    }

    fn process_internal(&mut self, input: &[f32], output: &mut [f32], sample_rate: f32) {
        self.sample_rate = sample_rate;

        let intensity = self.intensity.value;

        for (i, sample) in input.iter().enumerate() {
            let lfo_value = self.lfo.process();
            let delay_samples = 10.0 + lfo_value * 5.0; // Variable delay for pitch modulation

            self.delay_line.set_delay(delay_samples);
            let delayed = self.delay_line.process_internal(*sample);

            output[i] = *sample * (1.0 - intensity) + delayed * intensity;
        }
    }
}

impl EffectModule for Vibrato {
    fn name(&self) -> &str {
        &self.name
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        self.process_internal(input, output, self.sample_rate);
    }

    fn reset(&mut self) {
        self.delay_line.clear();
        self.lfo.reset();
    }
}
