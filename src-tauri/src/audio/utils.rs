// Utility functions for audio processing

use super::device::*;
use super::constants::DEFAULT_SAVE_PATH;
use std::path::PathBuf;

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

pub fn save_audio_buffer_to_file(buffer: &[f32], sample_rate: u32, channels: u16) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let save_path = next_save_path();
    let mut writer = hound::WavWriter::create(&save_path, spec)?;

    for &sample in buffer {
        let scaled_sample = (sample * i16::MAX as f32) as i16;
        writer.write_sample(scaled_sample)?;
    }

    println!("Audio saved to {}", save_path.display());

    writer.finalize()?;
    Ok(())
}

fn next_save_path() -> PathBuf {
    let dir = std::env::temp_dir();
    let stem = DEFAULT_SAVE_PATH.trim_end_matches(".wav");

    for index in 1.. {
        let filename = format!("{}{}.wav", stem, index);
        let path = dir.join(filename);
        if !path.exists() {
            return path;
        }
    }

    dir.join(DEFAULT_SAVE_PATH)
}