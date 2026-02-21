
use super::modules::utils::{ParameterValue, EffectParameter};

pub trait EffectModule: Send {
    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]);
    fn reset(&mut self);
    fn name(&self) -> &str; 
    fn set_parameter(&mut self, parameter: ParameterValue) -> anyhow::Result<()>;
    fn get_parameters(&self, name: &str) -> Vec<EffectParameter>;
}

pub trait EffectChain {
    fn reset_chain_state(&mut self);
    fn apply_processing(&mut self, in_b: &[f32], out_b: &mut [f32]);
    fn append_effect(&mut self, effect: Box<dyn EffectModule>);
    fn remove_effect_at(&mut self, index: usize) -> Option<Box<dyn EffectModule>>;
    fn remove_effect_from_name(&mut self, name: &str) -> Option<Box<dyn EffectModule>>;
    fn set_effect_parameter(&mut self, effect_name: &str, parameter: ParameterValue) -> anyhow::Result<()>;
}

pub trait FilterModule: Send {
    fn process(&mut self, sample: f32) -> f32;
    fn reset(&mut self);
}

pub trait FilterChain {
    fn reset_chain_state(&mut self);
    fn apply_processing(&mut self, in_b: &[f32], out_b: &mut [f32]);
    fn append_filter(&mut self, filter: Box<dyn FilterModule>);
    fn pop_filter(&mut self) -> Option<Box<dyn FilterModule>>;
    fn remove_filter_at(&mut self, index: usize) -> Option<Box<dyn FilterModule>>;
}