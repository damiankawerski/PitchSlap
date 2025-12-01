use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
pub struct AudioSpectrum {
    pub magnitudes: Vec<f32>,
    pub delta: Vec<f32>,  // Delta względem poprzedniej ramki (-1..1)
    pub frequencies: Vec<f32>,
    pub sample_rate: usize,
    pub timestamp: u64,
}

pub struct FFTProcessor {
    fft_planner: Arc<Mutex<RealFftPlanner<f32>>>,
    fft_size: usize,
    sample_rate: usize,
    last_sent: Arc<Mutex<Instant>>,
    throttle_interval: Duration,

    // ostatnie widmo (znormalizowane), do obliczania delty
    prev_spectrum: Arc<Mutex<Option<Vec<f32>>>>,

    // Noise floor tracking
    noise_floor: Arc<Mutex<Vec<f32>>>,
    noise_gate_db: f32,
    min_db: f32,
    max_db: f32,
}

impl FFTProcessor {
    pub fn new(sample_rate: usize, fft_size: usize, fps: u32) -> Self {
        Self {
            fft_planner: Arc::new(Mutex::new(RealFftPlanner::new())),
            fft_size,
            sample_rate,
            last_sent: Arc::new(Mutex::new(Instant::now())),
            throttle_interval: Duration::from_millis(1000 / fps as u64),
            noise_floor: Arc::new(Mutex::new(vec![0.0; fft_size / 2 + 1])),
            noise_gate_db: -60.0, // Wszystko poniżej -60dB traktuj jako szum
            min_db: -80.0,        // Dolna granica
            max_db: 0.0,          // Górna granica
            prev_spectrum: Arc::new(Mutex::new(None)),
        }
    }

    /// Oblicza deltę między obecnym a poprzednim widmem (PRZED normalizacją!)
    /// Zwraca: (current_normalized, delta_normalized)
    pub fn compute_delta_spectrum(&self, magnitudes_db: &[f32]) -> (Vec<f32>, Vec<f32>) {
        let mut prev_spectrum = self.prev_spectrum.lock().unwrap();

        // Oblicz deltę w dB (przed normalizacją)
        let delta_db: Vec<f32> = if let Some(ref prev) = *prev_spectrum {
            magnitudes_db
                .iter()
                .zip(prev.iter())
                .map(|(&curr, &prev)| curr - prev)
                .collect()
        } else {
            vec![0.0; magnitudes_db.len()]
        };

        // Zaktualizuj poprzednie widmo
        *prev_spectrum = Some(magnitudes_db.to_vec());

        // Normalizuj oba do 0-1
        let current_normalized = self.normalize_magnitudes(magnitudes_db);
        
        // Normalizuj deltę do -1..1 (negatywna delta = spadek, pozytywna = wzrost)
        let delta_normalized = self.normalize_delta(&delta_db);

        (current_normalized, delta_normalized)
    }

    /// Normalizuje deltę do zakresu -1..1
    fn normalize_delta(&self, delta: &[f32]) -> Vec<f32> {
        let max_abs = delta.iter()
            .map(|v| v.abs())
            .fold(0.0f32, f32::max)
            .max(1.0); // unikaj dzielenia przez 0

        delta.iter()
            .map(|&v| (v / max_abs).clamp(-1.0, 1.0))
            .collect()
    }

    pub fn process_and_send(
        &self,
        app_handle: &tauri::AppHandle,
        processed: &[f32],
    ) -> anyhow::Result<()> {
        {
            let mut last_sent = self.last_sent.lock().unwrap();
            if last_sent.elapsed() < self.throttle_interval {
                return Ok(());
            }
            *last_sent = Instant::now();
        }

        // Sprawdź czy mamy wystarczająco danych
        if processed.len() < self.fft_size {
            return Ok(());
        }

        // Oblicz FFT (compute_fft teraz oblicza też deltę)
        let spectrum = self.compute_fft(&processed[..self.fft_size])?;

        // Wyślij do frontendu (spectrum zawiera magnitudes + delta)
        app_handle.emit("audio-spectrum", &spectrum)?;

        Ok(())
    }

    fn stabilize_fft(&self, magnitudes: &mut [f32]) {
        // Minimalny próg energii, który MUSI zostać osiągnięty,
        // inaczej traktujemy to jako szum/kwaśne resztki
        const ABSOLUTE_FLOOR_DB: f32 = -75.0;

        // Median smoothing window (usuwa random-peaki)
        let mut smoothed = magnitudes.to_vec();

        for i in 2..magnitudes.len() - 2 {
            let mut window = [
                magnitudes[i - 2],
                magnitudes[i - 1],
                magnitudes[i],
                magnitudes[i + 1],
                magnitudes[i + 2],
            ];
            window.sort_by(|a, b| a.partial_cmp(b).unwrap());
            smoothed[i] = window[2]; // wartość mediany
        }

        // Hard clamp — usuwa resztę pierdolenia na dole
        for (i, mag) in smoothed.iter_mut().enumerate() {
            if *mag < ABSOLUTE_FLOOR_DB {
                *mag = ABSOLUTE_FLOOR_DB;
            }

            // Wycinamy bin 0-2 bo tam jest absolutny syf (DC + leakage)
            if i < 3 {
                *mag = ABSOLUTE_FLOOR_DB;
            }
        }

        magnitudes.copy_from_slice(&smoothed);
    }

    fn compute_fft(&self, samples: &[f32]) -> anyhow::Result<AudioSpectrum> {
        let mut input = samples.to_vec();

        // Zastosuj okno Hann
        Self::apply_hann_window(&mut input);

        // Przygotuj bufor wyjściowy
        let mut spectrum = vec![Complex::default(); self.fft_size / 2 + 1];

        // Wykonaj Real FFT
        let mut planner = self.fft_planner.lock().unwrap();
        let r2c = planner.plan_fft_forward(self.fft_size);
        r2c.process(&mut input, &mut spectrum)?;

        // Oblicz magnitude w dB
        let mut magnitudes: Vec<f32> = spectrum
            .iter()
            .map(|c| {
                let magnitude = c.norm();
                let db = 20.0 * (magnitude + 1e-10).log10();
                db.max(self.min_db)
            })
            .collect();

        // Zaktualizuj noise floor (exponential moving average)
        self.update_noise_floor(&magnitudes);

        // Usuń szum i zastosuj noise gate
        self.apply_noise_reduction(&mut magnitudes);

        self.stabilize_fft(&mut magnitudes);

        // WAŻNE: Oblicz deltę PRZED normalizacją (na wartościach dB)
        let (normalized, delta) = self.compute_delta_spectrum(&magnitudes);

        // Oblicz częstotliwości
        let frequencies: Vec<f32> = (0..spectrum.len())
            .map(|i| (i as f32 * self.sample_rate as f32) / self.fft_size as f32)
            .collect();

        Ok(AudioSpectrum {
            magnitudes: normalized,
            delta,
            frequencies,
            sample_rate: self.sample_rate,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    fn update_noise_floor(&self, magnitudes: &[f32]) {
        let mut noise_floor = self.noise_floor.lock().unwrap();
        let alpha = 0.02; // wolniejsza adaptacja = stabilniejszy background

        for (i, &db) in magnitudes.iter().enumerate() {
            let nf = &mut noise_floor[i];

            if *nf == 0.0 {
                *nf = db; // inicjalizacja
            } else {
                // Jeśli realny sygnał > floor o 6dB, nie aktualizujemy (to nie szum)
                if db <= *nf + 6.0 {
                    *nf = *nf * (1.0 - alpha) + db * alpha;
                }
            }
        }
    }

    fn apply_noise_reduction(&self, magnitudes: &mut [f32]) {
        let noise_floor = self.noise_floor.lock().unwrap();

        for (i, db) in magnitudes.iter_mut().enumerate() {
            let nf = noise_floor[i];

            // dynamiczne dopasowanie
            let gate = nf + 8.0; // 8dB ponad floor = sensowny sygnał

            if *db < gate {
                *db = nf; // przycisz do noise floor
            }
        }
    }

    fn normalize_magnitudes(&self, magnitudes: &[f32]) -> Vec<f32> {
        let max = magnitudes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let min = magnitudes.iter().cloned().fold(f32::INFINITY, f32::min);

        let range = (max - min).max(1.0);

        magnitudes
            .iter()
            .map(|&db| ((db - min) / range).clamp(0.0, 1.0))
            .collect()
    }

    fn apply_hann_window(buffer: &mut [f32]) {
        let n = buffer.len();
        for (i, sample) in buffer.iter_mut().enumerate() {
            let window =
                0.5 * (1.0 - ((2.0 * std::f32::consts::PI * i as f32) / (n - 1) as f32).cos());
            *sample *= window;
        }
    }

    // Metody konfiguracyjne

    /// Ustaw próg noise gate (w dB)
    pub fn set_noise_gate(&mut self, db: f32) {
        self.noise_gate_db = db;
    }

    /// Ustaw zakres dynamiczny (min/max dB)
    pub fn set_dynamic_range(&mut self, min_db: f32, max_db: f32) {
        self.min_db = min_db;
        self.max_db = max_db;
    }

    /// Zresetuj noise floor (użyteczne gdy zmienia się źródło audio)
    pub fn reset_noise_floor(&self) {
        let mut noise_floor = self.noise_floor.lock().unwrap();
        noise_floor.fill(0.0);
    }

    // Opcjonalna metoda do grupowania częstotliwości (mniej binów, bardziej czytelne)
    pub fn group_into_bands(spectrum: &AudioSpectrum, n_bands: usize) -> Vec<f32> {
        let bin_size = spectrum.magnitudes.len() / n_bands;

        (0..n_bands)
            .map(|i| {
                let start = i * bin_size;
                let end = ((i + 1) * bin_size).min(spectrum.magnitudes.len());

                // Średnia z grupy binów
                let sum: f32 = spectrum.magnitudes[start..end].iter().sum();
                sum / (end - start) as f32
            })
            .collect()
    }
}
