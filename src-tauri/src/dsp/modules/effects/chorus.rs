use crate::dsp::modules::filters::DelayLine;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;
use crate::dsp::modules::utils::lfo::LFO;
use crate::dsp::traits::EffectModule;

#[derive(Debug, Clone)]
pub struct Chorus {
    name: String,
    depth: EffectParameter,
    mix: EffectParameter,
    delay_line: DelayLine,
    lfo: LFO,
    sample_rate: f32,
}

impl Chorus {
    /// Creates a new chorus effect for thickening vocal texture.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Effect parameters including rate, depth, mix, and feedback
    ///
    /// # Returns
    ///
    /// A new `ChorusEffect` instance with modulated delay line and LFO.
    pub fn new(
        sample_rate: usize,
        depth: f32,
        mix: f32,
    ) -> Self {
        let effect = Self {
            name: "chorus".to_string(),
            depth: EffectParameter::new("depth", depth, 0.0, 1.0),
            mix: EffectParameter::new("mix", mix, 0.0, 1.0),
            delay_line: DelayLine::new(1024, 512.0, 0.5),
            lfo: LFO::new(0.5, 0.5, sample_rate as f32),
            sample_rate: sample_rate as f32,
        };

        effect
    }

    pub fn process_internal(&mut self, input: &[f32], output: &mut [f32], sample_rate: f32) {
        self.sample_rate = sample_rate;

        for (i, sample) in input.iter().enumerate() {
            let lfo_value = self.lfo.process();
            let delay_samples = 20.0 + lfo_value * self.depth.value * 10.0; // Variable delay

            self.delay_line.set_delay(delay_samples);
            let delayed = self.delay_line.process_internal(*sample);

            output[i] = *sample * (1.0 - self.mix.value) + delayed * self.mix.value;
        }
    }
}

impl EffectModule for Chorus {
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
