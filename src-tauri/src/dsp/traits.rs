pub trait EffectModule: Send {
    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]);
    fn reset_state(&mut self);
}

pub trait EffectChain {
    fn reset_chain_state(&mut self);
    fn apply_processing(&mut self, in_b: &[f32], out_b: &mut [f32]);
    fn append_effect(&mut self, effect: Box<dyn EffectModule>);
    fn pop_effect(&mut self) -> Option<Box<dyn EffectModule>>;
    fn remove_effect_at(&mut self, index: usize) -> Option<Box<dyn EffectModule>>;
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