use crate::dsp::processor::audio_processor::*;
use crate::dsp::effect_trait::AudioEffect;

pub struct Chorus {
    wet_level: f32,
    name: String,
}

impl Chorus {
    pub fn new(wet_level: f32) -> Self {
        Self { wet_level, name: "Chorus".into() }
    }
}

impl AudioEffect for Chorus {
    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32> {
        processor.process_chorus(input, self.wet_level)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
