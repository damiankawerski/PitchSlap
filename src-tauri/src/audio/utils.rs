// Utility functions for audio processing

use super::device::*;

pub fn verify_sample_rate(input: &AudioDevice, output: &AudioDevice) -> anyhow::Result<()> {
    if input.get_config().sample_rate != output.get_config().sample_rate {
        return Err(anyhow::anyhow!(
            "Input and output devices must have the same sample rate"
        ));
    }
    Ok(())
}

pub fn create_latency_samples(input: &AudioDevice, opt: &AudioDeviceOptions) -> usize {
    let latency = opt.get_latency();
    let latency_frames = (latency / 1_000.0 * input.get_config().sample_rate.0 as f32) as usize;
    let latency_samples = latency_frames * input.get_config().channels as usize;

    latency_samples
}
