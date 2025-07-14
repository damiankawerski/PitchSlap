// src-tauri/src/audio/utils.rs
// Utility functions for audio processing

use super::config::*;




// function to verify sample rate compatibility
pub fn verify_sample_rate(conf: &AudioDeviceConfig) -> anyhow::Result<()> {

    if conf.input_config().sample_rate != conf.output_config().sample_rate {
        return Err(anyhow::anyhow!("Input and output sample rates do not match"));
    }

    Ok(())
}


// Function to create latency samples based on the audio device configuration and latency in milliseconds
pub fn create_latency_samples(conf: &AudioDeviceConfig) -> usize {
    let latency = conf.latency();
    let latency_frames = (latency / 1_000.0) * conf.input_config().sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * conf.input_config().channels as usize;

    latency_samples   
}

