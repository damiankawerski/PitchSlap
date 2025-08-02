use super::processor::audio_processor::*;
use super::effect_trait::AudioEffect;

pub struct ModulationUnit {
    audio_processor: AudioProcessor,
    current_effect: Box<dyn AudioEffect>,
    is_active: bool,
}

impl ModulationUnit {
    pub fn new() -> Self {
        ModulationUnit {
            audio_processor: AudioProcessor::new(),
            current_effect: Box::new(AnimeVoice {}),
            is_active: false,
        }
    }

    
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_effect(&mut self, effect: Box<dyn AudioEffect>) {
        self.current_effect = effect;
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        if !self.is_active {
            return input.to_vec();
        }
        self.current_effect.process(input, &self.audio_processor)
    }
}