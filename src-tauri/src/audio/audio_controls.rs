// Controls for audio for frontend

use once_cell::sync::OnceCell;
use std::sync::Mutex;

use super::audio_handler::AudioHandler;
use super::device::AudioDeviceOptions;

pub struct AudioControls {
    audio_handler: AudioHandler,
    options: AudioDeviceOptions,
}

static AUDIO_CONTROLS: OnceCell<Mutex<AudioControls>> = OnceCell::new();

impl AudioControls {
    fn new() -> Self {
        let mut audio_handler = AudioHandler::new(AudioDeviceOptions::default());
        let options = AudioDeviceOptions::default();
        audio_handler.select_audio_devices(&options).expect("Failed to select audio devices");

        AudioControls {
            audio_handler,
            options,
        }
    }

    pub fn get_instance() -> &'static Mutex<AudioControls> {
        AUDIO_CONTROLS.get_or_init(|| Mutex::new(AudioControls::new()))
    }

    // Functions to get device lists
    pub fn get_input_devices_list(&self) -> Vec<String> {
        self.audio_handler.get_audio_devices().list_input_devices()
            .unwrap_or_else(|_| vec![])
    }

    pub fn get_output_devices_list(&self) -> Vec<String> {
        self.audio_handler.get_audio_devices().list_output_devices()
            .unwrap_or_else(|_| vec![])
    }

    pub fn get_virtual_devices_list(&self) -> Vec<String> {
        self.audio_handler.get_audio_devices().list_virtual_devices()
            .unwrap_or_else(|_| vec![])
    }

    // Get current options
    pub fn get_latency(&self) -> f32 {
        self.options.get_latency()
    }

    pub fn get_input_device(&self) -> String {
        self.options.get_input_device()
    }

    pub fn get_output_device(&self) -> String {
        self.options.get_output_device()
    }

    pub fn get_virtual_input(&self) -> String {
        self.options.get_virtual_input()
    }

    // Function change options
    pub fn set_input_device(&mut self, device_name: &str) -> anyhow::Result<()> {
        self.options.set_input_device(device_name);
        self.audio_handler.select_audio_devices(&self.options)
    }

    pub fn set_output_device(&mut self, device_name: &str) -> anyhow::Result<()> {
        self.options.set_output_device(device_name);
        self.audio_handler.select_audio_devices(&self.options)
    }

    pub fn set_virtual_input(&mut self, device_name: &str) -> anyhow::Result<()> {
        self.options.set_virtual_input(device_name);
        self.audio_handler.select_audio_devices(&self.options)
    }

    pub fn set_latency(&mut self, latency: f32) -> anyhow::Result<()> {
        self.options.set_latency(latency);
        self.audio_handler.select_audio_devices(&self.options)
    }

    // Switch audio engine modes
    pub fn start_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        self.audio_handler.start_audio_engine_loopback()
    }

    pub fn stop_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        self.audio_handler.stop_audio_engine_loopback()
    }

    pub fn start_audio_engine_throughput(&mut self) -> anyhow::Result<()> {
        self.audio_handler.start_audio_engine_throughput()
    }

    pub fn stop_audio_engine_throughput(&mut self) -> anyhow::Result<()> {
        self.audio_handler.stop_audio_engine_throughput()
    }

    // Audio engine status
    pub fn is_loopback_running(&self) -> bool {
        self.audio_handler.is_loopback_running()
    }

    pub fn is_throughput_running(&self) -> bool {
        self.audio_handler.is_throughput_running()
    }

    // Audio modulation controls
    pub fn enable_modulation(&mut self) -> anyhow::Result<()> {
        self.audio_handler.enable_modulation()
    }

    pub fn disable_modulation(&mut self) -> anyhow::Result<()> {
        self.audio_handler.disable_modulation()
    }

    pub fn get_effects_list(&self) -> Vec<String> {
        AudioHandler::get_effects_list()
    }

    pub fn set_effect(&mut self, effect_name: &str) -> anyhow::Result<()> {
        self.audio_handler.set_effect_from_string(effect_name)
    }

    pub fn is_modulation_active(&self) -> bool {
        self.audio_handler.is_modulation_active()
    }

    pub fn get_current_effect_name(&self) -> Option<String> {
        self.audio_handler.get_current_effect_name()
    }

    pub fn clear_effect(&mut self) -> anyhow::Result<()> {
        self.audio_handler.clear_effect()?;
        Ok(())
    }

    // App handle 

    pub fn set_app_handle(&mut self, handle: tauri::AppHandle) -> anyhow::Result<()> {
      self.audio_handler.set_app_handle(handle)
    }

    pub fn clear_app_handle(&mut self) -> anyhow::Result<()> {
      self.audio_handler.clear_app_handle()
    }
}


