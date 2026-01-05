use super::modules::visualizer::fft_visualizer::*;
use super::modules::filters::biquad::*;
use super::modules::filters::moving_average::*;
use super::modules::filters::de_esser::*;
use super::modules::filters::noise_gate::*;


pub struct AudioProcessor {
    sample_rate: usize,
    fft_visualizer: SpectrumVisualizer,
    buffer: Vec<f32>,
}


impl AudioProcessor {
    pub fn new(sample_rate: usize) -> Self {
        AudioProcessor {
            sample_rate,
            fft_visualizer: SpectrumVisualizer::new(sample_rate, 480, 60),
            buffer: Vec::new(),
        }
    }

    pub fn apply_de_esser(&self, input: &[f32], threshold: f32, ratio: f32) -> Vec<f32> {
        let mut de_esser = DeEsser::new(self.sample_rate as f32, threshold, ratio);
        input.iter().map(|&sample| de_esser.process(sample)).collect()
    }

    pub fn apply_noise_gate(&self, input: &[f32], threshold: f32, attack_ms: f32, release_ms: f32) -> Vec<f32> {
        let mut noise_gate = NoiseGate::new(threshold, attack_ms, release_ms, self.sample_rate as f32);
        input.iter().map(|&sample| noise_gate.process(sample)).collect()
    }

    pub fn apply_biquad_filter(&self, input: &[f32], coeffs: BiquadCoeffs) -> Vec<f32> {
        let mut biquad = BiquadFilter::new(coeffs);
        input.iter().map(|&sample| biquad.process(sample)).collect()
    }

    pub fn apply_moving_average(&self, input: &[f32], window_size: usize) -> Vec<f32> {
        let mut moving_average = MovingAverageFilter::new(window_size);
        input.iter().map(|&sample| moving_average.process(sample)).collect()
    }

    pub fn apply_default_processing(&mut self, input: &[f32]) -> Vec<f32> {
        // Apply all filters in sequence for optimal audio processing
        
        // 1. High-pass filter - usuwa niskie częstotliwości (stuknięcia, rumble)
        let hp_coeffs = highpass_coeffs(self.sample_rate as f32, 80.0, 0.707);
        self.buffer = self.apply_biquad_filter(input, hp_coeffs);
        
        // 2. Notch filter - wycina typowe częstotliwości sprzężenia (feedback)
        // Można dodać więcej notch filtrów dla różnych częstotliwości feedbacku
        let notch_coeffs = notch_coeffs(self.sample_rate as f32, 3000.0, 10.0);
        self.buffer = self.apply_biquad_filter(&self.buffer, notch_coeffs);
        
        // 3. Noise gate - usuwa cichsze szumy i stuknięcia (wyższy threshold)
        self.buffer = self.apply_noise_gate(&self.buffer, -35.0, 2.0, 30.0);
        
        // 4. De-esser - redukuje syczenie
        self.buffer = self.apply_de_esser(&self.buffer, -20.0, 3.0);
        
        // 5. Low-pass filter - wygładza górnę częstotliwości
        let lp_coeffs = lowpass_coeffs(self.sample_rate as f32, 12000.0, 0.707);
        self.buffer = self.apply_biquad_filter(&self.buffer, lp_coeffs);
        
        // 6. Moving average - końcowe wygładzenie
        self.buffer = self.apply_moving_average(&self.buffer, 2);
        
        self.buffer.clone()
    }

    pub fn process_and_send(&mut self, input: &[f32], app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
        let processed = self.apply_default_processing(input);
        self.fft_visualizer.emit_spectrum(app_handle, &processed)
    }

}