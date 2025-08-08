use crate::dsp::processor::audio_processor::*;
use crate::dsp::effect_trait::AudioEffect;

pub struct ChipmunkVoice {
    name: String,
}

impl ChipmunkVoice {
    pub fn new() -> Self {
        Self { name: "ChipmunkVoice".into() }
    }
}

impl AudioEffect for ChipmunkVoice {
    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32> {
        processor.set_pitch(2.5);
        
        let pitched = processor.process_pitch_shift(input);
        let distorted = processor.soft_clip(&pitched, 3.0);
        
        distorted.iter().map(|&x| (x * 0.6).clamp(-1.0, 1.0)).collect()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}