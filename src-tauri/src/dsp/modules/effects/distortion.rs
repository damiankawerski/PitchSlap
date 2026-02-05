use crate::dsp::traits::EffectModule;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;

pub struct Distortion {
    gain: EffectParameter,
}

impl Distortion {
    pub fn new(gain: f32) -> Self {
        Self { gain: EffectParameter::new("gain", gain, 0.0, 50.0) }
    }
}

impl EffectModule for Distortion {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        for (i, &sample) in input.iter().enumerate() {
            let driven = sample * self.gain.value;
            let distorted = driven.tanh();
            output[i] = distorted;
        }
    }

    fn reset(&mut self) {
        // No internal state to reset for the amplifier
    }

    fn name(&self) -> &str {
        "distortion"
    }
}