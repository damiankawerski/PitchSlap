use tauri::Emitter;
use super::audio_spectrum::*;

use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::dsp::modules::utils::windows::apply_hanning_window;

pub struct SpectrumVisualizer {
  fft_planner: Arc<Mutex<RealFftPlanner<f32>>>,
  fft_size: usize,
  sample_rate: usize,
  throttle_interval: Duration,
  last_emit: Arc<Mutex<Instant>>,
}

impl SpectrumVisualizer {
    pub fn new(sample_rate: usize, fft_size: usize, fps: u32) -> Self {
        Self {
            fft_planner: Arc::new(Mutex::new(RealFftPlanner::new())),
            fft_size,
            sample_rate,
            throttle_interval: Duration::from_millis(1000 / fps as u64),
            last_emit: Arc::new(Mutex::new(Instant::now())),
        }
    }

    fn normalize(&self, magnitudes_db: &[f32]) -> Vec<f32> {
        let min_db = -80.0;
        let max_db = 80.0;

        magnitudes_db
            .iter()
            .map(|&db| {
                if db <= min_db {
                    0.0
                } else if db >= max_db {
                    1.0
                } else {
                    (db - min_db) / (max_db - min_db)
                }
            })
            .collect()
    }

    fn estimate_pitch(&self, spectrum: &[Complex<f32>]) -> anyhow::Result<f32> {
        let mut max_magnitude = 0.0;
        let mut max_index = 0;

        for (i, c) in spectrum.iter().enumerate().skip(1) {
            let magnitude = c.norm();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
                max_index = i;
            }
        }

        let frequency = (max_index as f32 * self.sample_rate as f32) / self.fft_size as f32;
        Ok(frequency)
    }

    fn compute_fft(&self, samples: &[f32]) -> anyhow::Result<AudioFrame> {
        let mut input = samples.to_vec();

        apply_hanning_window(&mut input);

        let mut spectrum = vec![Complex::default(); self.fft_size / 2 + 1];

        let mut planner = self.fft_planner.lock().unwrap();
        let r2c = planner.plan_fft_forward(self.fft_size);
        r2c.process(&mut input, &mut spectrum)?;

        let magnitudes: Vec<f32> = spectrum
          .iter()
          .map(|c| {
            let magnitude = c.norm();
            let db = 20.0 * (magnitude + 1e-10).log10(); 
            db
          })
          .collect();

        let spec = self.normalize(&magnitudes);

        let frequencies: Vec<f32> = (0..spectrum.len())
            .map(|i| (i as f32 * self.sample_rate as f32) / self.fft_size as f32)
            .collect();
        
        let rms = (samples.iter().map(|&s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
        
        let pitch = self.estimate_pitch(&spectrum)?;

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
        // Sprawdź czy mamy wystarczająco danych
        if processed.len() < self.fft_size {
            return Ok(());
        }

        // Sprawdź throttling - czy minął wymagany czas od ostatniej emisji
        let mut last_emit = self.last_emit.lock().unwrap();
        let now = Instant::now();
        // if now.duration_since(*last_emit) < self.throttle_interval {
        //     return Ok(()); // Pomiń tę ramkę
        // }

        // Oblicz FFT (compute_fft teraz oblicza też deltę)
        let spectrum = self.compute_fft(&processed[..self.fft_size])?;
        app_handle.emit("audio-spectrum", &spectrum)?;

        // Zaktualizuj czas ostatniej emisji
        *last_emit = now;

        Ok(())
    }
}

