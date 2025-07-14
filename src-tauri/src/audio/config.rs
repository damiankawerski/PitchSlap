// src-tauri/src/audio/config.rs
// Struct to manage audio device configurations


use cpal::traits::{DeviceTrait};
use super::device::AudioDevices;


// Struct for selected audio devices configurations
pub struct AudioDeviceConfig {
    // Input device configuration
    input_device_conf: cpal::StreamConfig,
    // Output device configuration
    output_device_conf: cpal::StreamConfig,
    // Latency beetween input and output
    latency: f32,
}

// Implementation of the AudioDeviceConfig struct
impl AudioDeviceConfig {
    // Constructor for AudioDeviceConfig
    pub fn new(devices: &AudioDevices, latency: f32) -> anyhow::Result<Self> {
        let conf = AudioDeviceConfig { 
            input_device_conf: devices.input_device().default_input_config()?.into(), 
            output_device_conf: devices.output_device().default_output_config()?.into(),
            latency: latency};
        Ok(conf)
    }

    // Function to get the input device configuration
    pub fn input_config(&self) -> &cpal::StreamConfig {
        &self.input_device_conf
    }

    // Function to get the output device configuration
    pub fn output_config(&self) -> &cpal::StreamConfig {
        &self.output_device_conf
    }

    // Function to get the latency in milliseconds
    pub fn latency(&self) -> f32 {
        self.latency
    }

    // Function to set the input device configuration
    pub fn set_input_config(&mut self, config: cpal::StreamConfig) {
        self.input_device_conf = config;
    }

    // Function to set the output device configuration
    pub fn set_output_config(&mut self, config: cpal::StreamConfig) {
        self.output_device_conf = config;
    }

    // Function to set the latency
    pub fn set_latency(&mut self, latency: f32) {
        self.latency = latency;
    }
    
}

