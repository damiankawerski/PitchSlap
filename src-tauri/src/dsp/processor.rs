use super::modules::visualizer::fft_visualizer::*;
use super::modules::filters::biquad::*;
use super::modules::filters::moving_average::*;
use super::modules::filters::de_esser::*;
use super::modules::filters::noise_gate::*;


pub struct AudioProcessor {
    sample_rate: usize,
    fft_visualizer: SpectrumVisualizer,
    buffer: Vec<f32>,

    // Stateful effect chain (must persist between calls)
    hp: BiquadFilter,
    notch: BiquadFilter,
    noise_gate: NoiseGate,
    de_esser: DeEsser,
    lp: BiquadFilter,
    moving_average: MovingAverageFilter,
}


impl AudioProcessor {
    pub fn new(sample_rate: usize) -> Self {
        // Default chain parameters
        let hp_coeffs = highpass_coeffs(sample_rate as f32, 80.0, 0.707);
        let notch_coeffs = notch_coeffs(sample_rate as f32, 3000.0, 10.0);
        let lp_coeffs = lowpass_coeffs(sample_rate as f32, 12000.0, 0.707);

        let noise_gate = NoiseGate::new(-35.0, 2.0, 30.0, sample_rate as f32);
        let de_esser = DeEsser::new(sample_rate as f32, -20.0, 3.0);
        let moving_average = MovingAverageFilter::new(2);
        
        AudioProcessor {
            sample_rate,
            fft_visualizer: SpectrumVisualizer::new(sample_rate, 480, 30),
            buffer: Vec::new(),
            hp: BiquadFilter::new(hp_coeffs),
            notch: BiquadFilter::new(notch_coeffs),
            noise_gate,
            de_esser,
            lp: BiquadFilter::new(lp_coeffs),
            moving_average,
        }
    }

    pub fn reset_chain_state(&mut self) {
        self.hp.reset();
        self.notch.reset();
        self.noise_gate.reset();
        self.lp.reset();
        self.moving_average.reset();
    }

    pub fn set_noise_gate_threshold(&mut self, threshold: f32) {
        self.noise_gate.set_threshold(threshold);
    }

    pub fn apply_default_processing(&mut self, input: &[f32]) {
        // Fast path: resize only when needed (typically once at start)
        if self.buffer.len() != input.len() {
            self.buffer.resize(input.len(), 0.0);
        }
        
        // Single memcpy - unavoidable for safety
        self.buffer.copy_from_slice(input);

        // Prawdziwy chain efektów: jeden przebieg po buforze + stan utrzymany między wywołaniami.
        for sample in self.buffer.iter_mut() {
            let mut x = *sample;

            // 1. High-pass filter - usuwa niskie częstotliwości (stuknięcia, rumble)
            x = self.hp.process(x);

            // 2. Notch filter - wycina typowe częstotliwości sprzężenia (feedback)
            x = self.notch.process(x);

            // 3. Noise gate - usuwa cichsze szumy i stuknięcia
            x = self.noise_gate.process(x);

            // 4. De-esser - redukuje syczenie
            x = self.de_esser.process(x);

            // 5. Low-pass filter - wygładza górne częstotliwości
            x = self.lp.process(x);

            // 6. Moving average - końcowe wygładzenie
            x = self.moving_average.process(x);

            *sample = x;
        }
    }

    pub fn process_and_send(&mut self, input: &[f32], app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
        self.apply_default_processing(input);
        self.fft_visualizer.emit_spectrum(app_handle, &self.buffer)
    }

    pub fn test_processing(&mut self, input: &[f32]) -> Vec<f32> {
        self.apply_default_processing(input);
        self.buffer.clone()
    }

}