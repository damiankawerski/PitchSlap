use crate::dsp::traits::{EffectModule, EffectChain};

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
            effect.reset_state();
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
    }

    fn pop_effect(&mut self) -> Option<Box<dyn EffectModule>> {
        self.effects.pop()
    }

    fn remove_effect_at(&mut self, index: usize) -> Option<Box<dyn EffectModule>> {
        if index < self.effects.len() {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }
}