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

    fn get_parameters(&self) -> Vec<EffectParameter> {
        vec![self.gain.clone()]
    }

    fn set_parameter(&mut self, parameter: crate::dsp::modules::utils::ParameterValue) -> anyhow::Result<()> {
        match parameter.name.as_str() {
            "gain" => {
                self.gain.set_value(parameter.value);
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Unknown parameter: {}", parameter.name)),
        }
    }
}