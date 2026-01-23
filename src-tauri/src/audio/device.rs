use clap::Parser;
use cpal::{
    Device, Host,
    traits::{DeviceTrait, HostTrait},
};

// Structure for defining audio device options
#[derive(Parser, Debug)]
pub struct AudioDeviceOptions {
    // Input audio device to use (for any input)
    #[arg(short = 'i', long, value_name = "IN", default_value_t = String::from("default"))]
    input_device: String,

    // Output audio device to use (for loopback)
    #[arg(short = 'o', long, value_name = "OUT", default_value_t = String::from("default"))]
    output_device: String,

    // VB-Cable audio device to use (for passthrough) Please ensure that VB-Cable is installed (This reads data from buffer and writes it to VB-Cable)
    #[arg(short = 'v', long, value_name = "VB", default_value_t = String::from("default"))]
    virtual_input: String, // This is the VB-Cable output device

    // Latency in seconds between input and output devices (default is 150ms)
    #[arg(short = 'l', long, value_name = "LAT", default_value_t = 150.0)]
    latency: f32,
}

impl AudioDeviceOptions {
    // Default constructor for AudioDeviceOptions
    pub fn default() -> Self {
        AudioDeviceOptions::parse()
    }

    // Constructor to create AudioDeviceOptions from command line arguments - should not be used in production - use default() instead and set options manually
    pub fn new(
        input_device: String,
        output_device: String,
        virtual_input: String,
        latency: f32,
    ) -> Self {
        AudioDeviceOptions {
            input_device,
            output_device,
            virtual_input,
            latency,
        }
    }

    // Getters
    pub fn get_input_device(&self) -> String {
        self.input_device.clone()
    }

    pub fn get_output_device(&self) -> String {
        self.output_device.clone()
    }

    pub fn get_virtual_input(&self) -> String {
        self.virtual_input.clone()
    }

    pub fn get_latency(&self) -> f32 {
        self.latency
    }

    // Setters
    pub fn set_input_device(&mut self, device: &str) {
        self.input_device = device.to_string();
    }

    pub fn set_output_device(&mut self, device: &str) {
        self.output_device = device.to_string();
    }

    pub fn set_virtual_input(&mut self, device: &str) {
        self.virtual_input = device.to_string();
    }

    pub fn set_latency(&mut self, latency: f32) {
        self.latency = latency;
    }
}

impl Clone for AudioDeviceOptions {
    fn clone(&self) -> Self {
        AudioDeviceOptions {
            input_device: self.input_device.clone(),
            output_device: self.output_device.clone(),
            virtual_input: self.virtual_input.clone(),
            latency: self.latency,
        }
    }
}

// Struct for managing audio device along with its config
pub struct AudioDevice {
    device: Device,
    config: cpal::StreamConfig,
}

impl AudioDevice {
    // Create a new AudioDevice from a given device and config
    pub fn new(device: Device, config: cpal::StreamConfig) -> Self {
        AudioDevice { device, config }
    }

    // Get the underlying cpal device
    pub fn get_device(&self) -> &Device {
        &self.device
    }

    // Get the stream configuration for the audio device
    pub fn get_config(&self) -> &cpal::StreamConfig {
        &self.config
    }

    // Get the name of the audio device
    pub fn get_name(&self) -> String {
        self.device
            .name()
            .unwrap_or_else(|_| "Unknown Device".to_string())
    }

    // Setters
    pub fn set_device(&mut self, device: Device) {
        self.device = device;
    }

    pub fn set_config(&mut self, config: cpal::StreamConfig) {
        self.config = config;
    }
}

pub struct AudioDeviceManager {
    host: Host,
    input_device: Option<AudioDevice>,
    output_device: Option<AudioDevice>,
    virtual_input: Option<AudioDevice>,
}

impl AudioDeviceManager {
    pub fn new(host: Host) -> Self {
        AudioDeviceManager {
            host,
            input_device: None,
            output_device: None,
            virtual_input: None,
        }
    }


    pub fn default() -> Self {
        let mut manager = AudioDeviceManager::new(cpal::default_host());
        manager
            .select_devices_from_options(&AudioDeviceOptions::default())
            .unwrap();
        manager
    }

    // Setters for audio devices
    pub fn set_input_device(&mut self, device: Option<Device>) -> anyhow::Result<()> {
        if device.is_none() {
            self.input_device = None;
            return Ok(());
        }
        let config = device.as_ref().unwrap().default_input_config()?.into();
        self.input_device = Some(AudioDevice::new(device.unwrap(), config));
        Ok(())
    }

    pub fn set_output_device(&mut self, device: Option<Device>) -> anyhow::Result<()> {
        if device.is_none() {
            self.output_device = None;
            return Ok(());
        }
        let config = device.as_ref().unwrap().default_output_config()?.into();
        self.output_device = Some(AudioDevice::new(device.unwrap(), config));
        Ok(())
    }

    pub fn set_virtual_input(&mut self, device: Option<Device>) -> anyhow::Result<()> {
        if device.is_none() {
            self.virtual_input = None;
            return Ok(());
        }
        let config = device.as_ref().unwrap().default_output_config()?.into();
        self.virtual_input = Some(AudioDevice::new(device.unwrap(), config));
        Ok(())
    }

    // Getters for audio devices
    pub fn get_input_device(&self) -> Option<&AudioDevice> {
        self.input_device.as_ref()
    }

    pub fn get_output_device(&self) -> Option<&AudioDevice> {
        self.output_device.as_ref()
    }

    pub fn get_virtual_input(&self) -> Option<&AudioDevice> {
        self.virtual_input.as_ref()
    }

    // Device listing methods
    pub fn list_input_devices(&self) -> anyhow::Result<Vec<String>> {
        let devices = self.host.input_devices()?;
        Ok(devices.map(|d| d.name().unwrap_or_default()).collect())
    }

    pub fn list_output_devices(&self) -> anyhow::Result<Vec<String>> {
        let devices = self.host.output_devices()?;
        Ok(devices.map(|d| d.name().unwrap_or_default()).collect())
    }

    pub fn list_virtual_devices(&self) -> anyhow::Result<Vec<String>> {
        let output_devices = self
            .host
            .output_devices()?
            .filter(|d| {
                d.name()
                    .unwrap_or_default()
                    .contains("VB-Audio Virtual Cable")
                    || d.name().unwrap_or_default().contains("BlackHole")
            })
            .map(|d| d.name().unwrap_or_default());

        Ok(output_devices.collect())
    }

    // Helper functions to select chosen audio devices from name
    pub fn select_input_device(&mut self, name: &str) -> anyhow::Result<()> {
        let mut result: Option<Device> = None;
        if name == "default" {
            result = self.host.default_input_device();
        } else {
            for device in self.host.input_devices()? {
                if device.name().unwrap_or_default() == name {
                    result = Some(device);
                    break;
                }
            }
        }
        self.set_input_device(result)?;
        Ok(())
    }

    pub fn select_output_device(&mut self, name: &str) -> anyhow::Result<()> {
        let mut result: Option<Device> = None;
        if name == "default" {
            result = self.host.default_output_device();
        } else {
            for device in self.host.output_devices()? {
                if device.name().unwrap_or_default() == name {
                    result = Some(device);
                    break;
                }
            }
        }
        self.set_output_device(result)?;
        Ok(())
    }

    pub fn select_virtual_input(&mut self, name: &str) -> anyhow::Result<()> {
        let mut result: Option<Device> = None;
        if name == "default" {
            result = self.host.default_output_device();
        } else {
            for device in self.host.output_devices()? {
                if device.name().unwrap_or_default() == name {
                    result = Some(device);
                    break;
                }
            }
        }
        self.set_virtual_input(result)?;
        Ok(())
    }

    pub fn select_devices_from_options(
        &mut self,
        options: &AudioDeviceOptions,
    ) -> anyhow::Result<()> {
        self.select_input_device(&options.get_input_device())?;
        self.select_output_device(&options.get_output_device())?;
        self.select_virtual_input(&options.get_virtual_input())?;
        Ok(())
    }
}
