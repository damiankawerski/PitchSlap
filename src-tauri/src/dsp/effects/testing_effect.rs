use crate::dsp::processor::audio_processor::*;
use crate::dsp::effect_trait::AudioEffect;

pub struct TestingVoice {
    name: String,
}

impl TestingVoice {
    pub fn new() -> Self {
        Self { name: "TestingVoice".into() }
    }
}

impl AudioEffect for TestingVoice {

    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32> {
    if let Some(output) = processor.process_formant_shift(input) {
        output.to_vec()
    } else {
        Vec::new()
    }
}


    fn get_name(&self) -> String {
        self.name.clone()
    }
} 