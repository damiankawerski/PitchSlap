// Stream-related functionality for audio processing

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;
use std::sync::{Arc, Mutex};


use super::device::*;
use super::buffer::*;


pub struct AudioStreams {
    audio_buffer: Arc<Mutex<AudioBuffer>>,
    input_stream: Stream,
    output_stream: Stream,
}

impl AudioStreams {
    pub fn new(input_device: &AudioDevice, output_device: &AudioDevice, buffer_size: usize) -> anyhow::Result<Self> {
        let audio_buffer = Arc::new(Mutex::new(AudioBuffer::new(buffer_size)));

        let buffer_input = Arc::clone(&audio_buffer);
        let buffer_output = Arc::clone(&audio_buffer);

        let input_channels = input_device.get_config().channels as usize;
        let output_channels = output_device.get_config().channels as usize;
        
        let input_stream = input_device.get_device().build_input_stream(
            input_device.get_config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let Ok(mut buffer) = buffer_input.lock() {
                    // Modify input signal here
                    if let Err(e) = buffer.buffer_write(data) {
                        eprintln!("Input callback error: {}", e);
                    }
                }
            },
            error_callback,
            None, // timeout
        )?;

        let output_stream = output_device.get_device().build_output_stream(
            output_device.get_config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if let Ok(mut buffer) = buffer_output.lock() {
                    if let Err(e) = buffer.buffer_read(data, input_channels, output_channels) {
                        eprintln!("Output callback error: {}", e);
                    }
                }
            },
            error_callback,
            None, // timeout
        )?;

        Ok(AudioStreams {
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

    pub fn get_audio_buffer(&self) -> Arc<Mutex<AudioBuffer>> {
        Arc::clone(&self.audio_buffer)
    }
}



fn error_callback(err: cpal::StreamError) {
    eprintln!("Audio stream error: {}", err);
}