use anyhow::Context;
use once_cell::sync::Lazy;
use rodio::source::UniformSourceIterator;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;

struct StartupSoundState {
    samples: Vec<f32>,
    pos: usize,
    active: bool,
}

static STARTUP_SOUND: Lazy<Mutex<StartupSoundState>> = Lazy::new(|| {
    Mutex::new(StartupSoundState {
        samples: Vec::new(),
        pos: 0,
        active: false,
    })
});

/// Loads (and primes) the startup sound for mixing.
///
/// `target_channels` and `target_sample_rate` should match the stream buffer you mix into.
pub fn start_startup_sound<P: AsRef<Path>>(
    file_path: P,
    target_channels: u16,
    target_sample_rate: u32,
) -> anyhow::Result<()> {
    let file_path = file_path.as_ref();

    let file = File::open(file_path)
        .with_context(|| format!("Failed to open sound file: {}", file_path.display()))?;
    let decoder = Decoder::new(BufReader::new(file)).context("Failed to decode audio file")?;

    // Resample + channel-normalize to match our audio callback buffer.
    let uniform = UniformSourceIterator::new(decoder, target_channels, target_sample_rate);
    let samples: Vec<f32> = uniform.collect();

    let mut state = STARTUP_SOUND
        .lock()
        .map_err(|e| anyhow::anyhow!("StartupSound mutex poisoned: {}", e))?;

    state.samples = samples;
    state.pos = 0;
    state.active = true;

    Ok(())
}

/// Mixes the startup sound into `buffer` in-place.
/// Returns `true` if it mixed any audio this call.
pub fn mix_startup_sound_in_place(buffer: &mut [f32], volume: f32) -> bool {
    let mut state = match STARTUP_SOUND.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    if !state.active || state.samples.is_empty() {
        return false;
    }

    let mut mixed_any = false;
    for out_sample in buffer.iter_mut() {
        if state.pos >= state.samples.len() {
            state.active = false;
            break;
        }

        *out_sample += state.samples[state.pos] * volume;
        state.pos += 1;
        mixed_any = true;
    }

    mixed_any
}