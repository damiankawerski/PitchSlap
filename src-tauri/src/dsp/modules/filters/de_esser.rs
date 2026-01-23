use super::biquad::*;
use crate::dsp::traits::FilterModule;

#[derive(Debug)]
pub struct DeEsser {
    detector: BiquadFilter,
    compressor_gain: f32,
    threshold: f32,
    ratio: f32,
    envelope: f32,
    attack: f32,
    release: f32,
}

impl DeEsser {
    pub fn new(sample_rate: f32, threshold: f32, ratio: f32) -> Self {
        // Detektor sybilancji (5-8 kHz)
        let detector_coeffs = bandpass_coeffs(sample_rate, 6500.0, 0.707);
        
        Self {
            detector: BiquadFilter::new(detector_coeffs),
            compressor_gain: 1.0,
            threshold,
            ratio,
            envelope: 0.0,
            attack: 0.99,
            release: 0.9999,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Detekcja poziomu sybilancji
        let detected = self.detector.process(input).abs();
        
        // Envelope follower
        if detected > self.envelope {
            self.envelope = self.attack * self.envelope + (1.0 - self.attack) * detected;
        } else {
            self.envelope = self.release * self.envelope + (1.0 - self.release) * detected;
        }

        // Kompresja gdy przekroczony próg
        if self.envelope > self.threshold {
            let over = self.envelope / self.threshold;
            self.compressor_gain = 1.0 / (1.0 + (over - 1.0) * self.ratio);
        } else {
            self.compressor_gain = 1.0;
        }

        input * self.compressor_gain
    }
}

impl FilterModule for DeEsser {
    fn process(&mut self, input: f32) -> f32 {
        self.process(input)
    }

    fn reset(&mut self) {
        self.detector.reset();
        self.envelope = 0.0;
        self.compressor_gain = 1.0;
    }
}