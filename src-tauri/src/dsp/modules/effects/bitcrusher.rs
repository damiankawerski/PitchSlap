use crate::dsp::traits::EffectModule;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;
pub struct Bitcrusher {
    bit_depth: EffectParameter,
    sample_rate_reduction: usize,
    counter: usize,
    last_sample: f32,
}


impl Bitcrusher {
    pub fn new(bit_depth: f32, sample_rate_reduction: usize) -> Self {
        Self {
            bit_depth: EffectParameter::new("bit_depth", bit_depth, 1.0, 16.0),
            sample_rate_reduction,
            counter: 0,
            last_sample: 0.0,
        }
    }

    fn process_sample(&mut self, sample: f32) -> f32 {
        let max_amplitude = (1 << (self.bit_depth.value as u32 - 1)) as f32;
        (sample * max_amplitude).round() / max_amplitude
    }
}

impl EffectModule for Bitcrusher {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        if self.sample_rate_reduction == 0 {
            for (i, &sample) in input.iter().enumerate() {
                output[i] = self.process_sample(sample);
            }
            return;
        }

        for (i, &sample) in input.iter().enumerate() {
            if self.counter % self.sample_rate_reduction == 0 {
                let quantized = self.process_sample(sample);
                self.last_sample = quantized;
            }
            output[i] = self.last_sample;
            self.counter += 1;
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.last_sample = 0.0;
    }
    fn name(&self) -> &str {
        "bitcrusher"
    }
}