use realfft::ComplexToReal;
use realfft::RealToComplex;
use realfft::RealToComplexEven;
use realfft::ComplexToRealEven;
use realfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::f32::consts::PI;
use std::f32::consts::TAU;

// Stała reprezentująca liczbę zespoloną zero
const COMPLEX_ZERO: Complex<f32> = Complex::new(0.0, 0.0);

pub struct FormantShifter {
    forward_fft: RealToComplexEven<f32>,    // FFT dla sygnału wejściowego
    inverse_fft: ComplexToRealEven<f32>,    // Odwrotne FFT
    fft_scratch: Vec<Complex<f32>>,         // Bufor tymczasowy dla FFT
    fft_real: Vec<f32>,                      // Bufor na dane czasowe
    fft_cplx: Vec<Complex<f32>>,             // Bufor na dane w dziedzinie częstotliwości
    windowing: Vec<f32>,                     // Okno Hanninga
    frame_size: usize,                       // Rozmiar ramki FFT
    overlap: usize,                          // Liczba próbek nakładania
    sample_rate: usize,                      // Częstotliwość próbkowania
    in_fifo: Vec<f32>,                        // Bufor FIFO wejściowy
    out_fifo: Vec<f32>,                       // Bufor FIFO wyjściowy
    output_accumulator: Vec<f32>,            // Akumulator do overlap-add
}

impl FormantShifter {
    // Konstruktor – tworzy nowy FormantShifter
    pub fn new(window_duration_ms: usize, sample_rate: usize) -> Self {
        // Obliczenie rozmiaru ramki w próbkach (parzysta liczba)
        let mut frame_size = sample_rate * window_duration_ms / 1000;
        frame_size += frame_size % 2;
        let half_frame = frame_size / 2 + 1;

        // Tworzenie FFT i IFFT
        let mut planner = FftPlanner::new();
        let forward_fft = RealToComplexEven::new(frame_size, &mut planner);
        let inverse_fft = ComplexToRealEven::new(frame_size, &mut planner);

        // Bufory scratch wymagane przez realfft
        let ffft_len = forward_fft.get_scratch_len();
        let ifft_len = inverse_fft.get_scratch_len();
        let scratch_len = ffft_len.max(ifft_len);

        // Tworzenie okna Hanninga
        let mut windowing = vec![0.0; frame_size];
        for k in 0..frame_size {
            windowing[k] = -0.5 * (TAU * (k as f32) / (frame_size as f32)).cos() + 0.5;
        }

        // Inicjalizacja struktury
        Self {
            forward_fft,
            inverse_fft,
            fft_scratch: vec![COMPLEX_ZERO; scratch_len],
            fft_real: vec![0.0; frame_size],
            fft_cplx: vec![COMPLEX_ZERO; half_frame],
            windowing,
            frame_size,
            overlap: 32,  // domyślny overlap
            sample_rate,
            in_fifo: vec![0.0; frame_size],
            out_fifo: vec![0.0; frame_size],
            output_accumulator: vec![0.0; frame_size * 2],
        }
    }

    /// Przesuwa formanty w sygnale
    /// shift_factor > 1 -> podnosi formanty, < 1 -> obniża
    pub fn shift_formants(&mut self, shift_factor: f32, input: &[f32], output: &mut [f32]) {
        let step = self.frame_size / 4; // typowy overlap
        let half_frame = self.frame_size / 2 + 1;

        // Offset dla bezpieczeństwa indeksowania wyjścia
        let output_offset = if self.frame_size > step {
            self.frame_size - step
        } else {
            0
        };

        for i in 0..output.len() {
            // Wstawienie próbki do bufora FIFO
            self.in_fifo[self.overlap] = input[i];

            // Odczyt próbki z wyjścia z bezpiecznym indeksem
            let output_index = if self.overlap >= output_offset {
                self.overlap - output_offset
            } else {
                0
            };
            output[i] = self.out_fifo[output_index];

            self.overlap += 1;

            // Gdy FIFO jest pełne, przetwarzamy ramkę
            if self.overlap >= self.frame_size {
                self.overlap = self.frame_size - step;

                // Nakładanie okna Hanninga
                for k in 0..self.frame_size {
                    self.fft_real[k] = self.in_fifo[k] * self.windowing[k];
                }

                // FFT – zamiana sygnału czasowego na widmo
                let _ = self.forward_fft.process_with_scratch(
                    &mut self.fft_real,
                    &mut self.fft_cplx,
                    &mut self.fft_scratch,
                );

                // Tworzymy nowe widmo z przesuniętymi formantami
                let mut new_spectrum = vec![COMPLEX_ZERO; half_frame];
                for k in 0..half_frame {
                    let new_index = (k as f32 * shift_factor).round() as usize;
                    if new_index < half_frame {
                        new_spectrum[new_index] = self.fft_cplx[k];
                    }
                }
                self.fft_cplx.copy_from_slice(&new_spectrum);

                // Odwrotne FFT – z powrotem do dziedziny czasowej
                let _ = self.inverse_fft.process_with_scratch(
                    &mut self.fft_cplx,
                    &mut self.fft_real,
                    &mut self.fft_scratch,
                );

                // Overlap-add: sumowanie ramek
                let norm = 2.0 / half_frame as f32;
                for k in 0..self.frame_size {
                    self.output_accumulator[k] += self.fft_real[k] * self.windowing[k] * norm;
                }

                // Aktualizacja bufora wyjściowego i akumulatora
                self.out_fifo[..step].copy_from_slice(&self.output_accumulator[..step]);
                self.output_accumulator.copy_within(step..(step + self.frame_size), 0);
                self.in_fifo.copy_within(step..self.frame_size, 0);
            }
        }
    }
}
