// Module for audio device management
// This module handles audio device operations such as listing available devices,

use clap::Parser;
use cpal::{traits::{DeviceTrait, HostTrait}, Device, Host};


// Struct for selected audio devices
#[derive(Parser, Debug)]
pub struct AudioDeviceOpt {
    // The input audio device to use
    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    input_device: String,

    // The output audio device to use
    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    output_device: String,

    // Specify the delay between input and output
    #[arg(short, long, value_name = "DELAY_MS", default_value_t = 150.0)]
    latency: f32,
}

// Implementation of the AudioDeviceOpt struct for handling audio devices
impl AudioDeviceOpt {
    // Constructor for AudioDeviceOpt
    pub fn new(input_device: String, output_device: String, latency: f32) -> Self {
        AudioDeviceOpt {
            input_device,
            output_device,
            latency,
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

    // Function to get the latency in milliseconds
    pub fn latency(&self) -> f32 {
        self.latency
    }

    // Function to select available input audio device (no selection logic implemented yet)
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

    // Function to select available output audio device (no selection logic implemented yet)
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
}


