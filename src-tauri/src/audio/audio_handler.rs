// This is audio handler, other modules should use this module to interact with audio processing

use anyhow::Ok;
use cpal::Host;

use super::config::*;
use super::device::*;
use super::engine::*;


use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{JoinHandle};

struct AudioHandlerOpt {
    host: Arc<Host>,
    // Audio device options
    audio_device_opt: AudioDeviceOpt,
}

impl AudioHandlerOpt {
    fn new() -> Self {
        AudioHandlerOpt {
            host: Arc::new(cpal::default_host()),
            audio_device_opt: AudioDeviceOpt::new("default".to_string(), "default".to_string()),
        }
    }

    pub fn get_input_devices(&self) -> anyhow::Result<Vec<String>> {
        self.audio_device_opt.list_input_devices(&self.host)
    }

    pub fn get_output_devices(&self) -> anyhow::Result<Vec<String>> {
        self.audio_device_opt.list_output_devices(&self.host)
    }

    pub fn set_host(&mut self, host: Arc<Host>) {
        self.host = host;
    }

    pub fn get_host(&self) -> Arc<Host> {
        self.host.clone()
    }

    pub fn get_audio_device_opt(&self) -> &AudioDeviceOpt {
        &self.audio_device_opt
    }

    pub fn set_audio_device_opt(&mut self, opt: AudioDeviceOpt) {
        self.audio_device_opt = opt;
    }

}


pub struct AudioHandler {
    devices: AudioDevices,
    config: AudioDeviceConfig,
    audio_engine: AudioEngine,
    audio_handler_opt: AudioHandlerOpt,
}


impl AudioHandler {
    pub fn new() -> anyhow::Result<Self> {
        let audio_handler_opt = AudioHandlerOpt::new();
        let host = audio_handler_opt.get_host();
        let devices = AudioDevices::select_devices(host.as_ref(), &audio_handler_opt.get_audio_device_opt())?;
        let config = AudioDeviceConfig::new(&devices, 150.0)?;
        let audio_engine = AudioEngine::new(&devices, &config)?;

        Ok(Self {
            devices,
            config,
            audio_engine,
            audio_handler_opt,
        })
    }

    pub fn change_input_audio_device(&mut self, name: &str) -> anyhow::Result<()> {
        let host = self.audio_handler_opt.get_host();
        let opt = AudioDeviceOpt::new(name.to_string(), self.audio_handler_opt.get_audio_device_opt().output_device());
        let device = AudioDeviceOpt::select_input_device(host.as_ref(), &opt)?;
        self.devices.set_input_device(device);
        self.audio_handler_opt.set_audio_device_opt(opt);
        let new_config = AudioDeviceConfig::new(&self.devices, self.config.latency())?;
        self.audio_engine = AudioEngine::new(&self.devices, &self.config)?;
        self.config = new_config;
        Ok(())
    }

    pub fn change_output_audio_device(&mut self, name: &str) -> anyhow::Result<()> {
        let host = self.audio_handler_opt.get_host();
        let opt = AudioDeviceOpt::new(self.audio_handler_opt.get_audio_device_opt().input_device(), name.to_string());
        let device = AudioDeviceOpt::select_output_device(host.as_ref(), &opt)?;
        self.devices.set_output_device(device);
        self.audio_handler_opt.set_audio_device_opt(opt);
        let new_config = AudioDeviceConfig::new(&self.devices, self.config.latency())?;
        self.audio_engine = AudioEngine::new( &self.devices, &self.config)?;
        self.config = new_config;
        Ok(())
    }

    pub fn enable_input_stream(&self) -> anyhow::Result<()> {
        self.audio_engine.start_input_stream()
    }

    pub fn enable_output_stream(&self) -> anyhow::Result<()> {
        self.audio_engine.start_output_stream()
    }

    pub fn stop_input_stream(&self) -> anyhow::Result<()> {
        self.audio_engine.stop_input_stream()
    }

    pub fn stop_output_stream(&self) -> anyhow::Result<()> {
        self.audio_engine.stop_output_stream()
    }
    
}


pub static AUDIO_HANDLER: OnceLock<Mutex<Option<AudioHandler>>> = OnceLock::new();
pub static STREAM_THREAD: OnceLock<Arc<Mutex<Option<JoinHandle<Result<(), String>>>>>> = OnceLock::new();