use tauri::Emitter;
use super::audio_spectrum::*;

use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;
use std::sync::{Arc, Mutex};
use std::time::{Instant};

use crate::dsp::modules::utils::windows::apply_hanning_window;
use crate::dsp::modules::yin_detector::detector::yin::YINDetector;
use crate::dsp::modules::yin_detector::detector::PitchDetector;

/// Noise gate -- all values are zeroed if RMS is below this threshold (to cut off noise).
const NOISE_GATE_RMS: f32 = 0.002;
///  Bottom dB for normalization
const NOISE_FLOOR_DB: f32 = -70.0;
/// Top dB for normalization (0 dB = pełna skala, -10 dB = 1/3 skali, itp.)
const MAX_DB: f32 = -10.0;
/// Współczynnik wygładzania EMA: 0 = brak reakcji, 1 = brak wygładzania.
const SMOOTHING_ALPHA: f32 = 0.3;
/// Liczba binów po liniowym próbkowaniu 
const NUM_BINS: usize = 256;
/// Dolna częstotliwość wykresu (najniższy głos ludzki ~80 Hz).
const FREQ_MIN: f32 = 80.0;
/// Górna częstotliwość wykresu (górna granica głosu ludzkiego ~8 kHz).
const FREQ_MAX: f32 = 8_000.0;

pub struct SpectrumVisualizer {
  fft_planner: Arc<Mutex<RealFftPlanner<f32>>>,
  fft_size: usize,
  sample_rate: usize,
  last_emit: Arc<Mutex<Instant>>,
  smoothed_spectrum: Arc<Mutex<Vec<f32>>>,
  yin_detector: Mutex<YINDetector<f32>>,
}

impl SpectrumVisualizer {
    pub fn new(sample_rate: usize, fft_size: usize) -> Self {
        Self {
            fft_planner: Arc::new(Mutex::new(RealFftPlanner::new())),
            fft_size,
            sample_rate,
            last_emit: Arc::new(Mutex::new(Instant::now())),
            smoothed_spectrum: Arc::new(Mutex::new(vec![0.0f32; NUM_BINS])),
            yin_detector: Mutex::new(YINDetector::new(fft_size, fft_size / 2)),
        }
    }

    /// Normalizuje wartości dB do zakresu 0-1, odcinając szum poniżej NOISE_FLOOR_DB.
    fn normalize(&self, magnitudes_db: &[f32]) -> Vec<f32> {
        let range = MAX_DB - NOISE_FLOOR_DB;
        magnitudes_db
            .iter()
            .map(|&db| {
                if db <= NOISE_FLOOR_DB {
                    0.0
                } else if db >= MAX_DB {
                    1.0
                } else {
                    (db - NOISE_FLOOR_DB) / range
                }
            })
            .collect()
    }

    fn compute_fft(&self, samples: &[f32]) -> anyhow::Result<AudioFrame> {
        let rms = (samples.iter().map(|&s| s * s).sum::<f32>() / samples.len() as f32).sqrt();

        let mut input = samples.to_vec();
        apply_hanning_window(&mut input);

        let mut spectrum = vec![Complex::default(); self.fft_size / 2 + 1];

        let mut planner = self.fft_planner.lock().unwrap();
        let r2c = planner.plan_fft_forward(self.fft_size);
        r2c.process(&mut input, &mut spectrum)?;
        drop(planner);

        // --- dB magnitudy dla wszystkich binów FFT ---
        let hz_per_bin = self.sample_rate as f32 / self.fft_size as f32;
        let magnitudes_db: Vec<f32> = spectrum
          .iter()
          .map(|c| {
            let magnitude = c.norm() / self.fft_size as f32;
            20.0 * (magnitude + 1e-10_f32).log10()
          })
          .collect();

        // --- Liniowe próbkowanie → NUM_BINS binów (FREQ_MIN – FREQ_MAX) ---
        let bin_start_global = (FREQ_MIN / hz_per_bin) as usize;
        let bin_end_global   = ((FREQ_MAX / hz_per_bin) as usize + 1).min(magnitudes_db.len() - 1);
        let total_bins = bin_end_global - bin_start_global;
        let step = total_bins as f32 / NUM_BINS as f32;

        let linear_db: Vec<f32> = (0..NUM_BINS).map(|i| {
            let b0 = bin_start_global + (i as f32 * step) as usize;
            let b1 = (bin_start_global + ((i + 1) as f32 * step) as usize).min(bin_end_global);
            let b1 = b1.max(b0 + 1).min(magnitudes_db.len() - 1);
            magnitudes_db[b0..=b1].iter().cloned().fold(f32::NEG_INFINITY, f32::max)
        }).collect();

        let mut normalized = self.normalize(&linear_db);

        // Noise gate: jeśli RMS jest poniżej progu, wyzeruj wszystkie wartości (odcięcie szumu).
        if rms < NOISE_GATE_RMS {
            for v in normalized.iter_mut() {
                *v = 0.0;
            }
        }

        let mut smoothed = self.smoothed_spectrum.lock().unwrap();
        for (s, n) in smoothed.iter_mut().zip(normalized.iter()) {
            *s = SMOOTHING_ALPHA * n + (1.0 - SMOOTHING_ALPHA) * *s;
        }
        let spec = smoothed.clone();
        drop(smoothed);

        // Częstotliwości centralne dla każdego binu (liniowo)
        let frequencies: Vec<f32> = (0..NUM_BINS)
            .map(|i| FREQ_MIN + (i as f32 + 0.5) * (FREQ_MAX - FREQ_MIN) / NUM_BINS as f32)
            .collect();

        let pitch = {
            let mut yin = self.yin_detector.lock().unwrap();
            yin.get_pitch(
                samples,
                self.sample_rate,
                0.01,  // power_threshold
                0.35,  // clarity_threshold
            )
            .map(|p| p.frequency)
            .unwrap_or(0.0)
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(AudioFrame::new(
            rms,
            pitch,
            spec,
            frequencies,
            timestamp,
        ))
    }

    pub fn emit_spectrum(
        &self,
        app_handle: &tauri::AppHandle,
        processed: &[f32],
    ) -> anyhow::Result<()> {
        if processed.len() < self.fft_size {
            return Ok(());
        }

        let mut last_emit = self.last_emit.lock().unwrap();
        let now = Instant::now();

        let spectrum = self.compute_fft(&processed[..self.fft_size])?;
        app_handle.emit("audio-spectrum", &spectrum)?;

        *last_emit = now;

        Ok(())
    }
}

