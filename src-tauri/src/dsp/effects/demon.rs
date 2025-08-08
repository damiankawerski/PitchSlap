use crate::dsp::processor::audio_processor::*;
use crate::dsp::effect_trait::AudioEffect;

pub struct DemonVoice {
    name: String,
}

impl DemonVoice {
    pub fn new() -> Self {
        Self { name: "DemonVoice".into() }
    }
}

impl AudioEffect for DemonVoice {
    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32> {
        processor.set_pitch(0.4);
        
        let pitched = processor.process_pitch_shift(input);
        let distorted = processor.soft_clip(&pitched, 2.0);
        let resonant = processor.process_resonator(&distorted);
        
        // Combine with some original for harmonics
        let result: Vec<f32> = distorted.iter().zip(resonant.iter())
            .map(|(&d, &r)| d + r * 0.3)
            .collect();
            
        result.iter().map(|&x| x.clamp(-1.0, 1.0)).collect()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}