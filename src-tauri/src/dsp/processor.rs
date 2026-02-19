#![allow(dead_code)]

use super::modules::chains::filters_chain::*;
use super::modules::chains::modulation_chain::*;

use super::modules::visualizer::fft_visualizer::*;
use super::modules::utils::ParameterValue;
use super::traits::{EffectChain, FilterChain};
use crate::dsp::modules::effects::*;
use crate::dsp::traits::EffectModule;
use super::effect_factory::create_effect_from_name;

pub struct AudioProcessor {
    fft_visualizer: SpectrumVisualizer,
    filters_chain: FiltersChain,
    modulation_chain: ModulationChain,
    sample_rate: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: usize) -> Self {
        let mut modulation_chain = ModulationChain::new();
        //modulation_chain.append_effect(Box::new(Reverb::new(sample_rate as u32, 1)));
        // modulation_chain.append_effect(Box::new(Chorus::new(
        //     sample_rate,
        //     50.0,
        //     50.0,
        // )));

        //modulation_chain.append_effect(Box::new(AutoTune::new(sample_rate as f32)));
        // modulation_chain.append_effect(Box::new(Reverb::new(sample_rate as u32, 1)));
        // modulation_chain.append_effect(Box::new(Amplifier::new(20.0)));
        // let mut auto_tune = AutoTune::new(sample_rate as f32);
        // auto_tune.set_scale(auto_tune::Scale::EMinor);
        // modulation_chain.append_effect(Box::new(auto_tune));

        modulation_chain.append_effect(Box::new(Distortion::new(50.0)));

        AudioProcessor {
            fft_visualizer: SpectrumVisualizer::new(sample_rate, 480),
            filters_chain: FiltersChain::new(),
            modulation_chain: modulation_chain,
            sample_rate,
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

    pub fn send_spectrum(&mut self, input: &[f32], app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
        self.fft_visualizer.emit_spectrum(app_handle, input)?;
        Ok(())
    }

    pub fn append_effect_from_name(&mut self, name: &str) -> anyhow::Result<()> {
        let effect = create_effect_from_name(name, self.sample_rate, 1)
            .map_err(anyhow::Error::msg)?;
        self.modulation_chain.append_effect(effect);
        Ok(())
    }

    pub fn remove_effect_from_name(&mut self, name: &str) -> Option<Box<dyn EffectModule>> {
        self.modulation_chain.remove_effect_from_name(name)
    }

    pub fn set_effect_parameter(&mut self, effect_name: &str, parameter: ParameterValue) -> anyhow::Result<()> {
        self.modulation_chain.set_effect_parameter(effect_name, parameter)
    }
}
