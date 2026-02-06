use super::internals::{autocorrelation, pitch_from_peaks, DetectorInternals, Pitch};
use super::PitchDetector;
use super::super::float::Float;
use super::super::utils::buffer::square_sum;
use super::super::utils::peak::PeakCorrection;


pub struct AutocorrelationDetector<T>
where
    T: Float,
{
    internals: DetectorInternals<T>,
}

impl<T> AutocorrelationDetector<T>
where
    T: Float,
{
    pub fn new(size: usize, padding: usize) -> Self {
        let internals = DetectorInternals::new(size, padding);
        AutocorrelationDetector { internals }
    }
}

impl<T> PitchDetector<T> for AutocorrelationDetector<T>
where
    T: Float + std::iter::Sum,
{
    fn get_pitch(
        &mut self,
        signal: &[T],
        sample_rate: usize,
        power_threshold: T,
        clarity_threshold: T,
    ) -> Option<Pitch<T>> {
        assert_eq!(signal.len(), self.internals.size);

        if square_sum(signal) < power_threshold {
            return None;
        }

        let result_ref = self.internals.buffers.get_real_buffer();
        let mut result_guard = result_ref.lock().unwrap();
        let result = &mut result_guard[..];

        autocorrelation(signal, &mut self.internals.buffers, result);
        let clarity_threshold = clarity_threshold * result[0];

        pitch_from_peaks(result, sample_rate, clarity_threshold, PeakCorrection::None)
    }
}
