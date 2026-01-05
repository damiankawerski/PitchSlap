use super::processor::AudioProcessor;

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
            is_active: true,
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

    pub fn process_and_send(&mut self, input: &[f32]) -> anyhow::Result<()> {
        if self.is_active {
            if let Some(ref handle) = self.app_handle {
                self.audio_processor.process_and_send(input, handle)?;
            }
        }
        Ok(())
    }


}
