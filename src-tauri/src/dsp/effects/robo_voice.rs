use crate::dsp::processor::audio_processor::*;
use crate::dsp::effect_trait::AudioEffect;

pub struct RoboVoice {
    name: String,
}

impl RoboVoice {
    pub fn new() -> Self {
        Self { name: "RoboVoice".into() }
    }
}

impl AudioEffect for RoboVoice {

    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32> {
        let ring_modulated = processor.process_ring_modulation(input);
        let bit_crushed = processor.process_bit_crush(&ring_modulated);
        let filtered = processor.process_lowpass(&bit_crushed);

        filtered.iter().map(|&x| (x * 0.7).clamp(-1.0, 1.0)).collect()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}