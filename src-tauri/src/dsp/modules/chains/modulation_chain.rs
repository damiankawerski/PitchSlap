use crate::dsp::modules::utils::ParameterValue;
use crate::dsp::traits::{EffectChain, EffectModule};

pub struct ModulationChain {
    effects: Vec<Box<dyn EffectModule>>,
}

impl ModulationChain {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

impl EffectChain for ModulationChain {
    fn reset_chain_state(&mut self) {
        for effect in self.effects.iter_mut() {
            effect.reset();
        }
    }

    fn apply_processing(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        debug_assert_eq!(in_b.len(), out_b.len());

        match self.effects.len() {
            0 => {
                out_b.copy_from_slice(in_b);
            }
            1 => {
                self.effects[0].process(in_b, out_b);
            }
            2 => {
                let mut intermediate = vec![0.0; in_b.len()];
                self.effects[0].process(in_b, &mut intermediate);
                self.effects[1].process(&intermediate, out_b);
            }
            _ => {
                let mut buf_a = vec![0.0; in_b.len()];
                let mut buf_b = vec![0.0; in_b.len()];

                let last_index = self.effects.len() - 1;

                enum InputSrc {
                    In,
                    A,
                    B,
                }

                let mut input_src = InputSrc::In;

                for (i, effect) in self.effects.iter_mut().enumerate() {
                    let is_last = i == last_index;

                    match input_src {
                        InputSrc::In => {
                            if is_last {
                                effect.process(in_b, out_b);
                                break;
                            }
                            effect.process(in_b, &mut buf_a);
                            input_src = InputSrc::A;
                        }
                        InputSrc::A => {
                            let input = &buf_a[..];
                            if is_last {
                                effect.process(input, out_b);
                                break;
                            }
                            effect.process(input, &mut buf_b);
                            input_src = InputSrc::B;
                        }
                        InputSrc::B => {
                            let input = &buf_b[..];
                            if is_last {
                                effect.process(input, out_b);
                                break;
                            }
                            effect.process(input, &mut buf_a);
                            input_src = InputSrc::A;
                        }
                    }
                }
            }
        }
    }

    fn append_effect(&mut self, effect: Box<dyn EffectModule>) {
        self.effects.push(effect);
        println!(
            "Current effects in chain: {:?}",
            self.effects.iter().map(|e| e.name()).collect::<Vec<_>>()
        );
    }

    fn remove_effect_from_name(&mut self, name: &str) -> Option<Box<dyn EffectModule>> {
        if let Some(pos) = self.effects.iter().position(|e| e.name() == name) {
            return Some(self.effects.remove(pos));
        } else {
            return None;
        }
    }

    fn remove_effect_at(&mut self, index: usize) -> Option<Box<dyn EffectModule>> {
        if index < self.effects.len() {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    fn set_effect_parameter(
        &mut self,
        effect_name: &str,
        parameter: ParameterValue,
    ) -> anyhow::Result<()> {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.name() == effect_name) {
            effect.set_parameter(parameter)
        } else {
            Err(anyhow::anyhow!(
                "Effect '{}' not found in chain",
                effect_name
            ))
        }
    }

    fn set_auto_tune_scale(
        &mut self,
        scale: crate::dsp::modules::effects::Scale,
    ) -> anyhow::Result<()> {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.name() == "AutoTune") {
            effect.set_scale(scale)
        } else {
            Err(anyhow::anyhow!("AutoTune effect not found in chain"))
        }
    }

    fn get_active_effects(&self) -> Vec<String> {
        self.effects.iter().map(|e| e.name().to_string()).collect()
    }

    fn get_effect_parameters(
        &self,
        effect_name: &str,
    ) -> anyhow::Result<Vec<crate::dsp::modules::utils::EffectParameter>> {
        if let Some(effect) = self.effects.iter().find(|e| e.name() == effect_name) {
            Ok(effect.get_parameters(effect_name))
        } else {
            Err(anyhow::anyhow!(
                "Effect '{}' not found in chain",
                effect_name
            ))
        }
    }
}
