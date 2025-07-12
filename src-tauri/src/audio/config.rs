// src-tauri/src/audio/config.rs
// Struct to manage audio device configurations


use cpal::traits::{DeviceTrait};
use cpal::Device;


// Struct for selected audio devices configurations
pub struct AudioDeviceConfig {
    // Input device configuration
    input_device_conf: cpal::StreamConfig,
    // Output device configuration
    output_device_conf: cpal::StreamConfig,
}

// Implementation of the AudioDeviceConfig struct
impl AudioDeviceConfig {
    // Constructor for AudioDeviceConfig
    pub fn new(input_device: Device, output_device: Device) -> anyhow::Result<Self> {
        let conf = AudioDeviceConfig { 
            input_device_conf: input_device.default_input_config()?.into(), 
            output_device_conf: output_device.default_output_config()?.into() };
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
}

