use super::processor::audio_processor::AudioProcessor;

pub trait AudioEffect: Send + Sync {
    fn process(&mut self, input: &[f32], processor: &mut AudioProcessor) -> Vec<f32>;
    fn get_name(&self) -> String;
}