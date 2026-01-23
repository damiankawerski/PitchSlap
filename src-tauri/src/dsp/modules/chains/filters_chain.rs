use crate::dsp::traits::{FilterChain, FilterModule};

pub struct FiltersChain {
    filters: Vec<Box<dyn FilterModule>>,
}

impl FiltersChain {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }   
}

impl FilterChain for FiltersChain {
    fn reset_chain_state(&mut self) {
        for filter in self.filters.iter_mut() {
            filter.reset();
        }
    }

    fn apply_processing(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        debug_assert_eq!(in_b.len(), out_b.len());

        for (i, sample) in in_b.iter().enumerate() {
            let mut processed_sample = *sample;
            for filter in self.filters.iter_mut() {
                processed_sample = filter.process(processed_sample);
            }
            out_b[i] = processed_sample;
        }
    }

    fn append_filter(&mut self, filter: Box<dyn FilterModule>) {
        self.filters.push(filter);
    }

    fn pop_filter(&mut self) -> Option<Box<dyn FilterModule>> {
        self.filters.pop()
    }

    fn remove_filter_at(&mut self, index: usize) -> Option<Box<dyn FilterModule>> {
        if index < self.filters.len() {
            Some(self.filters.remove(index))
        } else {
            None
        }
    }
}