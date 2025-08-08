// Audio processing engine - contains streams and devices

use crate::dsp::modulation_unit::ModulationUnit;
use std::sync::{Arc, Mutex};

use super::stream::*;
use super::device::*;
use super::utils::*;
pub struct AudioEngine {
    streams: AudioStreams,
}

impl AudioEngine {
    pub fn new(input_device: &AudioDevice, output_device: &AudioDevice, opt: &AudioDeviceOptions, modulation_unit: Option<Arc<Mutex<ModulationUnit>>>) -> anyhow::Result<Self> {
        // Verify sample rates match
        verify_sample_rate(&input_device, &output_device)?;
        // Create latency samples based on options
        let latency_samples = create_latency_samples(&input_device, opt);
        // Create audio streams with the specified buffer size
        let streams = AudioStreams::new(&input_device, &output_device, latency_samples, modulation_unit)?;
        Ok(AudioEngine {
            streams,
        })
    }

    pub fn start(&self) -> anyhow::Result<()> {
        // Start input and output streams
        self.streams.start_input_stream()?;
        self.streams.start_output_stream()?;
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        // Stop input and output streams
        self.streams.stop_input_stream()?;
        self.streams.stop_output_stream()?;
        Ok(())
    }
}