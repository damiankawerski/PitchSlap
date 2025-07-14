// src-tauri/src/audio/engine.rs
// This module handles audio processing tasks such as reading and writing audio data

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;
use std::sync::{Arc, Mutex};

use super::device::*;
use super::config::*;
use super::utils::*;
use super::buffer::*;

// Struct for the audio engine
// Tutaj dam komentarz po polsku
// audio_buffer istnieje tylko po to żeby był jawny dla zewnętrznego kodu
// Sam w sobie konstruktor używa wewnętrznego bufora do strumieni
// Ale to ten sam bufor który jest używany do wejścia i wyjścia !!!
// Dostęp do bufora jest możliwy przez get_buffer


// Modification of signal will probably need changing this struct
pub struct AudioStreams {
    audio_buffer: Arc<Mutex<AudioBuffer>>, // Shared audio buffer for input and output throws dead_code but is used in closures
    input_stream: Stream,
    output_stream: Stream,
}

// Implementation of the AudioStreams struct
impl AudioStreams {
    // Constructor for AudioStreams
    pub fn new(devices: &AudioDevices, configs: &AudioDeviceConfig, buffer_size: usize) -> anyhow::Result<Self> {
        // Verify sample rate compatibility
        verify_sample_rate(&configs)?;
        
        // Create shared audio buffer
        let audio_buffer = Arc::new(Mutex::new(AudioBuffer::new(buffer_size)));
        
        // Create input and output streams

        let input_device = devices.input_device();
        let output_device = devices.output_device();

        // Clone buffer for closures
        let buffer_input = Arc::clone(&audio_buffer);
        let buffer_output = Arc::clone(&audio_buffer);
        
        // Get channel counts from configs
        let input_channels = configs.input_config().channels as usize;
        let output_channels = configs.output_config().channels as usize;

        // Create input stream
        let input_stream = input_device.build_input_stream(
            configs.input_config(),
            move |data: &[f32], info: &cpal::InputCallbackInfo| {
                if let Ok(mut buffer) = buffer_input.lock() {
                    if let Err(e) = buffer.input_data_fn(data, info) {
                        eprintln!("Input callback error: {}", e);
                    }
                }
            },
            error_callback,
            None, // timeout
        )?;

        // Create output stream
        let output_stream = output_device.build_output_stream(
            configs.output_config(),
            move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                if let Ok(mut buffer) = buffer_output.lock() {
                    if let Err(e) = buffer.output_data_fn(input_channels, output_channels, data, info) {
                        eprintln!("Output callback error: {}", e);
                    }
                }
            },
            error_callback,
            None, // timeout
        )?;

        Ok(Self {
            audio_buffer,
            input_stream,
            output_stream,
        })
    }

    pub fn start_input_stream(&self) -> anyhow::Result<()> {
        self.input_stream.play()?;
        Ok(())
    }

    pub fn start_output_stream(&self) -> anyhow::Result<()> {
        self.output_stream.play()?;
        Ok(())
    }

    pub fn stop_input_stream(&self) -> anyhow::Result<()> {
        self.input_stream.pause()?;
        Ok(())
    }

    pub fn stop_output_stream(&self) -> anyhow::Result<()> {
        self.output_stream.pause()?;
        Ok(())
    }

    pub fn get_buffer(&self) -> Arc<Mutex<AudioBuffer>> {
        Arc::clone(&self.audio_buffer)
    }
}

// Error callback function
fn error_callback(err: cpal::StreamError) {
    eprintln!("Audio stream error: {}", err);
}


// Struct for main audio engine functionality
pub struct AudioEngine {
    audio_streams: AudioStreams,
}

// Implementation of the AudioEngine struct
impl AudioEngine {
    // Constructor for AudioEngine with default devices and configurations 
    pub fn new(devices: &AudioDevices, config: &AudioDeviceConfig) -> anyhow::Result<Self> {

        let capacity = create_latency_samples(&config);

        Ok(Self {
            audio_streams: AudioStreams::new(&devices, &config, capacity)?,
        })
    }

    pub fn start_input_stream(&self) -> anyhow::Result<()> {
        self.audio_streams.start_input_stream()
    }

    pub fn start_output_stream(&self) -> anyhow::Result<()> {
        self.audio_streams.start_output_stream()
    }

    pub fn stop_input_stream(&self) -> anyhow::Result<()> {
        self.audio_streams.stop_input_stream()
    }

    pub fn stop_output_stream(&self) -> anyhow::Result<()> {
        self.audio_streams.stop_output_stream()
    }
}

