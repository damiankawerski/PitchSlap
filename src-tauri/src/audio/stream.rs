// Stream-related functionality for audio processing

use cpal::Stream;
use cpal::traits::{DeviceTrait, StreamTrait};
use std::sync::{Arc, Mutex};

use super::buffer::*;
use super::device::*;

use crate::dsp::modulation_unit::ModulationUnit;

pub struct AudioStreams {
    audio_buffer: Arc<Mutex<AudioBuffer>>,
    input_stream: Stream,
    output_stream: Stream,
}

impl AudioStreams {
    pub fn new(
        input_device: &AudioDevice,
        output_device: &AudioDevice,
        buffer_size: usize,
        modulation_unit: Option<Arc<Mutex<ModulationUnit>>>,
    ) -> anyhow::Result<Self> {
        let audio_buffer = Arc::new(Mutex::new(AudioBuffer::new(buffer_size)));

        let buffer_input = Arc::clone(&audio_buffer);
        let buffer_output = Arc::clone(&audio_buffer);

        let input_channels = input_device.get_config().channels as usize;
        let output_channels = output_device.get_config().channels as usize;

        let input_stream = input_device.get_device().build_input_stream(
            input_device.get_config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let Ok(mut buffer) = buffer_input.lock() {
                        let processed = if let Some(ref modulation_unit) = modulation_unit {
                        match modulation_unit.lock() {
                            Ok(mut mod_unit) => {
                                // Here apply modulation effects
                                // Now implementing fft visualizer so off
                                // mod_unit.process(data)

                                mod_unit.process_and_send(data).unwrap_or_else(|e| {
                                    eprintln!("Error processing modulation unit: {}", e);
                                    Vec::new()
                                })
                            },
                            Err(poisoned) => {
                                eprintln!("⚠️ modulation_unit mutex poisoned — recovering.");
                                poisoned.into_inner().process_and_send(data).unwrap_or_else(|e| {
                                    eprintln!("Error processing modulation unit: {}", e);
                                    Vec::new()
                                })
                            }
                        }
                    } else {
                        data.to_vec()
                    };

                    if let Err(e) = buffer.buffer_write(&processed) {
                        eprintln!("Input callback error: {}", e);
                    }
                }
            },
            error_callback,
            None,
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
            None,
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
