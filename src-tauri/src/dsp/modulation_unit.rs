use super::processor::AudioProcessor;
use super::traits::EffectModule;
use crate::dsp::modules::utils::ParameterValue;

// current effect to option
pub struct ModulationUnit {
    audio_processor: AudioProcessor,
    is_active: bool,
    app_handle: Option<tauri::AppHandle>,
}

impl ModulationUnit {
    pub fn new(sample_rate: usize) -> Self {
        ModulationUnit {
            audio_processor: AudioProcessor::new(sample_rate),
            is_active: false,
            app_handle: None,
        }
    }

    pub fn set_app_handle(&mut self, handle: tauri::AppHandle) {
        self.app_handle = Some(handle);
    }

    pub fn clear_app_handle(&mut self) {
        self.app_handle = None;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn append_effect_from_name(&mut self, name: &str) -> anyhow::Result<()> {
        self.audio_processor.append_effect_from_name(name)
    }

     pub fn remove_effect_from_name(&mut self, name: &str) -> Option<Box<dyn EffectModule>> {
        self.audio_processor.remove_effect_from_name(name)
    }

    pub fn set_effect_parameter(&mut self, effect_name: &str, parameter: ParameterValue) -> anyhow::Result<()> {
        self.audio_processor.set_effect_parameter(effect_name, parameter)
    }

    pub fn get_effect_parameters(&self, effect_name: &str) -> anyhow::Result<Vec<crate::dsp::modules::utils::EffectParameter>> {
        self.audio_processor.get_effect_parameters(effect_name)
    }

    pub fn set_auto_tune_scale(&mut self, scale: crate::dsp::modules::effects::auto_tune::Scale) -> anyhow::Result<()> {
        self.audio_processor.set_auto_tune_scale(scale)
    }

    pub fn get_auto_tune_scale(&self) -> Option<crate::dsp::modules::effects::auto_tune::Scale> {
        self.audio_processor.get_auto_tune_scale()
    }

    pub fn get_active_effects(&self) -> Vec<String> {
        self.audio_processor.get_active_effects()
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        if self.is_active {
            if let Some(app_handle) = &self.app_handle {
                let _ = self.audio_processor.process_and_send(input, app_handle);
            }
            self.audio_processor.process(input)
        } else {
            if let Some(app_handle) = &self.app_handle {
                let _ = self.audio_processor.send_spectrum(input, app_handle);
            }
            input.to_vec()
        }
    }

    pub fn is_app_handle_set(&self) -> bool {
        self.app_handle.is_some()
    }
}
