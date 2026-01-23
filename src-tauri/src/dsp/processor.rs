use super::modules::visualizer::fft_visualizer::*;
use super::modules::filters::biquad::*;
use super::modules::filters::moving_average::*;
use super::modules::filters::de_esser::*;
use super::modules::filters::noise_gate::*;
use super::modules::chains::filters_chain::*;
use super::modules::chains::modulation_chain::*;
use super::traits::{EffectChain, FilterChain};
use super::modules::phase_vocoder::pitch_shifter::*;

pub struct AudioProcessor {
    fft_visualizer: SpectrumVisualizer,
    filters_chain: FiltersChain,
    modulation_chain: ModulationChain,
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
        let mut filters_chain = FiltersChain::new();
        filters_chain.append_filter(Box::new(BiquadFilter::new(hp_coeffs)));
        filters_chain.append_filter(Box::new(BiquadFilter::new(notch_coeffs)));
        filters_chain.append_filter(Box::new(noise_gate));
        filters_chain.append_filter(Box::new(de_esser));
        filters_chain.append_filter(Box::new(BiquadFilter::new(lp_coeffs)));
        filters_chain.append_filter(Box::new(moving_average));

        let mut modulation_chain = ModulationChain::new();
        modulation_chain.append_effect(Box::new(PitchShifter::new(
            40,
            sample_rate,
            4.0,
            4,
        )));
        
        AudioProcessor {
            fft_visualizer: SpectrumVisualizer::new(sample_rate, 480, 30),
            filters_chain,
            modulation_chain,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        let mut filtered_output = vec![0.0; input.len()];
        self.filters_chain
            .apply_processing(input, &mut filtered_output);

        let mut modulated_output = vec![0.0; input.len()];
        self.modulation_chain
            .apply_processing(&filtered_output, &mut modulated_output);

        modulated_output
    }

    pub fn process_and_send(&mut self, input: &[f32], app_handle: &tauri::AppHandle) -> Vec<f32> {
        let output = self.process(input);

        self.fft_visualizer.emit_spectrum(app_handle, &output).ok();

        output
    }
}