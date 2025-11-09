use super::effect_factory::EFFECTS;
use super::effect_trait::AudioEffect;
use super::processor::audio_processor::*;

// current effect to option
pub struct ModulationUnit {
    audio_processor: AudioProcessor,
    current_effect: Option<Box<dyn AudioEffect>>,
    is_active: bool,
}

impl ModulationUnit {
    pub fn new(sample_rate: usize) -> Self {
        ModulationUnit {
            audio_processor: AudioProcessor::new(sample_rate),
            current_effect: None,
            is_active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_effect(&mut self, effect: Box<dyn AudioEffect>) {
        self.current_effect = Some(effect);
    }

    pub fn clear_effect(&mut self) {
        self.current_effect = None;
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        if !self.is_active || self.current_effect.is_none() {
            return input.to_vec();
        }
        self.current_effect
            .as_mut()
            .unwrap()
            .process(input, &mut self.audio_processor)
    }

    pub fn get_effects_list() -> Vec<String> {
        EFFECTS.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_current_effect_name(&self) -> Option<String> {
        self.current_effect
            .as_ref()
            .and_then(|effect| Some(effect.get_name()))
    }

    pub fn set_from_string(&mut self, effect_name: &str) {
        let effects = EFFECTS.lock().unwrap();
        if let Some(factory) = effects.get(effect_name) {
            self.set_effect(factory());
        } else {
            eprintln!("Effect '{}' not found", effect_name);
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
