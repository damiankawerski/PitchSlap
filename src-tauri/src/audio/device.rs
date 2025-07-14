// Module for audio device management
// This module handles audio device operations such as listing available devices,

use clap::Parser;
use cpal::{traits::{DeviceTrait, HostTrait}, Device, Host};


// Struct for selecting audio devices
#[derive(Parser, Debug)]
pub struct AudioDeviceOpt {
    // The input audio device to use
    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    input_device: String,

    // The output audio device to use
    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    output_device: String,
}

// Implementation of the AudioDeviceOpt struct for handling audio devices
impl AudioDeviceOpt {
    // Constructor for AudioDeviceOpt
    pub fn new(input_device: String, output_device: String) -> Self {
        AudioDeviceOpt {
            input_device,
            output_device,
        }
    }
    // Function to get the input audio device
    pub fn input_device(&self) -> String {
        self.input_device.clone()
    }

    // Function to get the output audio device
    pub fn output_device(&self) -> String {
        self.output_device.clone()
    }
    // Setters for the audio device options
    pub fn set_input_device(&mut self, device: String) {
        self.input_device = device;
    }

    pub fn set_output_device(&mut self, device: String) {
        self.output_device = device;
    }

    // Function to select available input audio device 
    pub fn select_input_device(host: &Host, opt: &Self) -> anyhow::Result<Device> {

        if opt.input_device == "default" {
            return Ok(host.default_input_device().ok_or_else(|| anyhow::anyhow!("No default input device found"))?);
        } else {
            let devices = host.input_devices()?;
            for device in devices {
                if device.name()? == opt.input_device {
                    return Ok(device);
                }
            }
            Err(anyhow::anyhow!("Input device '{}' not found", opt.input_device))
        }
    }

    // Function to select available output audio device 
    pub fn select_output_device(host: &Host, opt: &Self) -> anyhow::Result<Device> {

        if opt.output_device == "default" {
            return Ok(host.default_output_device().ok_or_else(|| anyhow::anyhow!("No default output device found"))?);
        } else {
            let devices = host.output_devices()?;
            for device in devices {
                if device.name()? == opt.output_device {
                    return Ok(device);
                } 
            }
            Err(anyhow::anyhow!("Output device '{}' not found", opt.output_device))
        }
    }

    pub fn list_input_devices(&self, host: &Host) -> anyhow::Result<Vec<String>> {
        let devices = host.input_devices()?;
        let device_names: Vec<String> = devices.map(|d| d.name().unwrap_or_default()).collect();
        Ok(device_names)
    }

    pub fn list_output_devices(&self, host: &Host) -> anyhow::Result<Vec<String>> {
        let devices = host.output_devices()?;
        let device_names: Vec<String> = devices.map(|d| d.name().unwrap_or_default()).collect();
        Ok(device_names)
    }

}



// Struct to hold audio devices
pub struct AudioDevices {
    input_device: Device,
    output_device: Device,
}

impl AudioDevices {
    pub fn new(input_device: Device, output_device: Device) -> Self {
        AudioDevices {
            input_device,
            output_device,
        }
    }

    pub fn input_device(&self) -> &Device {
        &self.input_device
    }

    pub fn output_device(&self) -> &Device {
        &self.output_device
    }

    pub fn set_input_device(&mut self, device: Device) {
        self.input_device = device;
    }

    pub fn set_output_device(&mut self, device: Device) {
        self.output_device = device;
    }

    pub fn select_devices(host: &Host, opt: &AudioDeviceOpt) -> anyhow::Result<Self> {
        let input_device = AudioDeviceOpt::select_input_device(&host, opt)?;
        let output_device = AudioDeviceOpt::select_output_device(&host, opt)?;
        Ok(AudioDevices::new(input_device, output_device))
    }
}


