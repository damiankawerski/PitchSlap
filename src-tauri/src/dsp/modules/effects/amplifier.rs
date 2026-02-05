use crate::dsp::traits::EffectModule;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;
pub struct Amplifier {
    gain: EffectParameter,
}

impl Amplifier {
    pub fn new(gain: f32) -> Self {
        Self { gain: EffectParameter::new("gain", gain, 0.0, 50.0) }
    }
}

impl EffectModule for Amplifier {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        for (i, &sample) in input.iter().enumerate() {
            output[i] = sample * self.gain.value;
        }
    }

    fn reset(&mut self) {
        // No internal state to reset for the amplifier
    }

    fn name(&self) -> &str {
        "amplifier"
    }
}