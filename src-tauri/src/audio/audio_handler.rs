use super::device::*;
use super::engine::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crate::dsp::modulation_unit::ModulationUnit;

pub struct AudioHandler {
    options: AudioDeviceOptions,
    audio_devices: AudioDeviceManager,

    loopback_handle: Option<JoinHandle<()>>,
    loopback_control: Option<Arc<Mutex<bool>>>, // true = run, false = stop
    loopback_running: bool,

    throughput_handle: Option<JoinHandle<()>>,
    throughput_control: Option<Arc<Mutex<bool>>>, // true = run, false = stop
    throughput_running: bool,

    modulation_unit: Option<Arc<Mutex<ModulationUnit>>>,
}

impl AudioHandler {
    // default host for now
    pub fn new(options: AudioDeviceOptions) -> Self {
        AudioHandler {
            audio_devices: AudioDeviceManager::new(cpal::default_host()),
            options: options,

            loopback_handle: None,
            loopback_control: None,
            loopback_running: false,
            
            throughput_handle: None,
            throughput_control: None,
            throughput_running: false,

            modulation_unit: Some(Arc::new(Mutex::new(ModulationUnit::new(
                444100.0, // Default sample rate, can be changed later
            )))),
        }
    }

    // Getter for current audio devices
    pub fn get_audio_devices(&self) -> &AudioDeviceManager {
        &self.audio_devices
    }

    fn restart(&mut self) -> anyhow::Result<()> {
        if self.loopback_running {
            self.stop_audio_engine_loopback()?;
            self.start_audio_engine_loopback()?;
        }

        if self.throughput_running {
            self.stop_audio_engine_throughput()?;
            self.start_audio_engine_throughput()?;
        }

        Ok(())
    }

    // Use this every time options are changed - make sure to call this after changing options
    pub fn select_audio_devices(&mut self, opt: &AudioDeviceOptions) -> anyhow::Result<()> {
        self.audio_devices.select_devices_from_options(opt)?;
        self.options = opt.clone();
        self.modulation_unit = Some(Arc::new(Mutex::new(ModulationUnit::new(self.audio_devices.get_input_device().unwrap().get_config().sample_rate.0 as f32))));
        // Restart engine if it is running
        self.restart()?;

        Ok(())
    } 

    // Modulation unit methods
    pub fn get_modulation_unit(&self) -> Option<Arc<Mutex<ModulationUnit>>> {
        if let Some(ref unit) = self.modulation_unit {
            Some(Arc::clone(unit))
        } else {
            None
        }
    }

    pub fn enable_modulation(&mut self) -> anyhow::Result<()> {
        if let Some(ref unit) = self.modulation_unit {
            let mut unit = unit.lock().unwrap();
            unit.set_active(true);
        }
        self.restart()?;

        Ok(())
    }

    pub fn disable_modulation(&mut self) -> anyhow::Result<()> {
        if let Some(ref unit) = self.modulation_unit {
            let mut unit = unit.lock().unwrap();
            unit.set_active(false);
        }
        self.restart()?;

        Ok(())
    }

    pub fn get_effects_list() -> Vec<String> {
        ModulationUnit::get_effects_list()
    }

    pub fn set_effect_from_string(&mut self, effect_name: &str) -> anyhow::Result<()> {
        if let Some(ref unit) = self.modulation_unit {
            let mut unit = unit.lock().unwrap();
            unit.set_from_string(effect_name);
        }
        self.restart()?;

        Ok(())
    }

    pub fn is_modulation_active(&self) -> bool {
        if let Some(ref unit) = self.modulation_unit {
            let unit = unit.lock().unwrap();
            unit.is_active()
        } else {
            false
        }
    }
    
    pub fn get_current_effect_name(&self) -> Option<String> {
        if let Some(ref unit) = self.modulation_unit {
            let unit = unit.lock().unwrap();
            unit.get_current_effect_name()
        } else {
            None
        }
    }

    pub fn clear_effect(&mut self) -> anyhow::Result<()> {
        if let Some(ref unit) = self.modulation_unit {
            let mut unit = unit.lock().unwrap();
            unit.clear_effect();
        }
        self.restart()?;

        Ok(())
    }

    // Start and stop audio engine for loopback mode
    pub fn start_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        if self.loopback_running {
            return Err(anyhow::anyhow!("Loopback audio engine is already running"));
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
        self.loopback_control = Some(Arc::clone(&control));
        
        let mut modulation_unit_clone: Option<Arc<Mutex<ModulationUnit>>> = None;
        if self.modulation_unit.is_some() {
            modulation_unit_clone = Some(Arc::clone(self.modulation_unit.as_ref().unwrap()));
        }

        // Spawn audio processing thread
        let handle = thread::spawn(move || {
            Self::audio_engine_thread(
                input_device_clone,
                output_device_clone,
                options_clone,
                control,
                modulation_unit_clone,
            );
        });

        self.loopback_handle = Some(handle);
        self.loopback_running = true;

        Ok(())
    }

    pub fn stop_audio_engine_loopback(&mut self) -> anyhow::Result<()> {
        if !self.loopback_running {
            return Err(anyhow::anyhow!("Loopback audio engine is not running"));
        }

        // Signal the thread to stop
        if let Some(ref control) = self.loopback_control {
            if let Ok(mut should_run) = control.lock() {
                *should_run = false;
            }
        }

        // Wait for thread to finish
        if let Some(handle) = self.loopback_handle.take() {
            handle.join()
                .map_err(|_| anyhow::anyhow!("Failed to join audio thread"))?;
        }

        self.loopback_control = None;
        self.loopback_running = false;

        Ok(())
    }

    pub fn start_audio_engine_throughput(&mut self) -> anyhow::Result<()> {
        if self.throughput_running {
            return Err(anyhow::anyhow!("Throughput audio engine is already running"));
        }

        // Verify we have required devices
        let input_device = self.audio_devices.get_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
        let output_device = self.audio_devices.get_virtual_input()
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
        self.throughput_control = Some(Arc::clone(&control));
        let mut modulation_unit_clone: Option<Arc<Mutex<ModulationUnit>>> = None;
        if self.modulation_unit.is_some() {
            modulation_unit_clone = Some(Arc::clone(self.modulation_unit.as_ref().unwrap()));
        }

        // Spawn audio processing thread
        let handle = thread::spawn(move || {
            Self::audio_engine_thread(
                input_device_clone,
                output_device_clone,
                options_clone,
                control,
                modulation_unit_clone,
            );
        });

        self.throughput_handle = Some(handle);
        self.throughput_running = true;

        Ok(())
    }

    pub fn stop_audio_engine_throughput(&mut self) -> anyhow::Result<()> {
        if !self.throughput_running {
            return Err(anyhow::anyhow!("Throughput audio engine is not running"));
        }

        // Signal the thread to stop
        if let Some(ref control) = self.throughput_control {
            if let Ok(mut should_run) = control.lock() {
                *should_run = false;
            }
        }

        // Wait for thread to finish
        if let Some(handle) = self.throughput_handle.take() {
            handle.join()
                .map_err(|_| anyhow::anyhow!("Failed to join audio thread"))?;
        }

        self.throughput_control = None;
        self.throughput_running = false;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.loopback_running || self.throughput_running
    }

    pub fn is_loopback_running(&self) -> bool {
        self.loopback_running
    }

    pub fn is_throughput_running(&self) -> bool {
        self.throughput_running
    }

    pub fn get_status(&self) -> String {
        match (self.loopback_running, self.throughput_running) {
            (true, true) => "both_running".to_string(),
            (true, false) => "loopback_running".to_string(),
            (false, true) => "throughput_running".to_string(),
            (false, false) => "stopped".to_string(),
        }
    }

    fn audio_engine_thread(
        input_device: AudioDevice,
        output_device: AudioDevice,
        options: AudioDeviceOptions,
        control: Arc<Mutex<bool>>,
        modulation_unit: Option<Arc<Mutex<ModulationUnit>>>,
    ) {

        // Create the audio engine
        let audio_engine = match AudioEngine::new(&input_device, &output_device, &options, modulation_unit) {
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

    }


}

impl Drop for AudioHandler {
    fn drop(&mut self) {
        let _ = self.stop_audio_engine_loopback();
        let _ = self.stop_audio_engine_throughput();
    }
}