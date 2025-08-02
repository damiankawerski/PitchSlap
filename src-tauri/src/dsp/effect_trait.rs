use super::processor::audio_processor::AudioProcessor;

pub trait AudioEffect: Send + Sync {
    fn process(&mut self, input: &[f32], processor: &AudioProcessor) -> Vec<f32>;
}