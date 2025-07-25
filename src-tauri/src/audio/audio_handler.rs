use clap::Parser;

use super::device::*;
use super::engine::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct AudioHandler {
    options: AudioDeviceOptions,
    audio_devices: AudioDeviceManager,
    engine_handle: Option<JoinHandle<()>>,
    engine_control: Option<Arc<Mutex<bool>>>, // true = run, false = stop
    is_running: bool,
}

impl AudioHandler {
    // default host for now
    pub fn new() -> Self {
        AudioHandler {
            audio_devices: AudioDeviceManager::new(cpal::default_host()),
            options: AudioDeviceOptions::parse(),
            engine_handle: None,
            engine_control: None,
            is_running: false,
        }
    }

    pub fn select_audio_devices(&mut self, opt: &AudioDeviceOptions) -> anyhow::Result<()> {
        self.audio_devices.select_devices_from_options(opt)?;
        self.options = opt.clone();
        Ok(())
    }

    pub fn start_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        if self.is_running {
            return Err(anyhow::anyhow!("Audio engine is already running"));
        }

        // Verify we have required devices
        let input_device = self.audio_devices.get_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
        let output_device = self.audio_devices.get_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        // Clone devices and options for the thread
        let input_device_clone = AudioDevice::new(
            input_device.get_device().clone(),
            input_device.get_config().clone()
        );
        let output_device_clone = AudioDevice::new(
            output_device.get_device().clone(),
            output_device.get_config().clone()
        );
        let options_clone = self.options.clone();

        // Create control flag
        let control = Arc::new(Mutex::new(true));
        self.engine_control = Some(Arc::clone(&control));

        // Spawn audio processing thread
        let handle = thread::spawn(move || {
            Self::audio_engine_thread(
                input_device_clone,
                output_device_clone,
                options_clone,
                control,
            );
        });

        self.engine_handle = Some(handle);
        self.is_running = true;

        println!("Audio engine loopback started");
        Ok(())
    }

    pub fn stop_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        if !self.is_running {
            return Err(anyhow::anyhow!("Audio engine is not running"));
        }

        // Signal the thread to stop
        if let Some(ref control) = self.engine_control {
            if let Ok(mut should_run) = control.lock() {
                *should_run = false;
            }
        }

        // Wait for thread to finish
        if let Some(handle) = self.engine_handle.take() {
            handle.join()
                .map_err(|_| anyhow::anyhow!("Failed to join audio thread"))?;
        }

        self.engine_control = None;
        self.is_running = false;

        println!("Audio engine loopback stopped");
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn audio_engine_thread(
        input_device: AudioDevice,
        output_device: AudioDevice,
        options: AudioDeviceOptions,
        control: Arc<Mutex<bool>>,
    ) {
        println!("Audio engine thread started");

        // Create the audio engine
        let audio_engine = match AudioEngine::new(&input_device, &output_device, &options) {
            Ok(engine) => engine,
            Err(e) => {
                eprintln!("Failed to create audio engine: {}", e);
                return;
            }
        };

        // Start the engine
        if let Err(e) = audio_engine.start() {
            eprintln!("Failed to start audio engine: {}", e);
            return;
        }

        println!("Audio engine started successfully");

        // Keep the engine running until stop signal
        loop {
            // Check if we should continue running
            let should_continue = match control.lock() {
                Ok(should_run) => *should_run,
                Err(_) => false, // If mutex is poisoned, stop
            };

            if !should_continue {
                break;
            }

            // Sleep to prevent busy waiting
            thread::sleep(Duration::from_millis(10));
        }

        // Stop the engine
        if let Err(e) = audio_engine.stop() {
            eprintln!("Failed to stop audio engine: {}", e);
        }

        println!("Audio engine thread terminated");
    }
}

impl Drop for AudioHandler {
    fn drop(&mut self) {
        let _ = self.stop_audio_engine_loopback();
    }
}