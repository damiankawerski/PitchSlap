//! Spatial audio effects for stereo enhancement and ambience.
//!
//! This module implements reverb, stereo width control, and spatial positioning
//! for enhanced audio experience.

use crate::dsp::traits::EffectModule;
use crate::dsp::modules::utils::effect_parameter::EffectParameter;
use crate::dsp::modules::filters::*;


pub struct Reverb {
    enabled: bool,
    room_size: EffectParameter, // Room size (0.0 - 1.0)
    damping: EffectParameter,   // High frequency damping
    wet_level: EffectParameter, // Wet signal level
    dry_level: EffectParameter, // Dry signal level
    width: EffectParameter,     // Stereo width

    all_pass_filters: Vec<AllPassFilter>,
    comb_filters: Vec<CombFilter>,
    sample_rate: u32,
    channels: usize,
}

impl Reverb {
    pub fn new(sample_rate: u32, channels: usize) -> Self {
        let mut reverb = Self {
            enabled: true,
            room_size: EffectParameter::new("room_size", 0.5, 0.0, 1.0),
            damping: EffectParameter::new("damping", 0.5, 0.0, 1.0),
            wet_level: EffectParameter::new("wet_level", 0.3, 0.0, 1.0),
            dry_level: EffectParameter::new("dry_level", 0.7, 0.0, 1.0),
            width: EffectParameter::new("width", 1.0, 0.0, 1.0),

            all_pass_filters: Vec::new(),
            comb_filters: Vec::new(),
            sample_rate,
            channels
        };

        reverb.initialize_filters();
        reverb
    }

    fn initialize_filters(&mut self) {
        let sample_rate = self.sample_rate as f32;

        // Comb filter delays (in samples) - tuned for natural reverb
        let comb_delays = [
            (sample_rate * 0.0297) as usize,
            (sample_rate * 0.0371) as usize,
            (sample_rate * 0.0411) as usize,
            (sample_rate * 0.0437) as usize,
            (sample_rate * 0.005) as usize,
            (sample_rate * 0.0017) as usize,
            (sample_rate * 0.0083) as usize,
            (sample_rate * 0.0109) as usize,
        ];

        // All-pass filter delays
        let allpass_delays = [
            (sample_rate * 0.005) as usize,
            (sample_rate * 0.0017) as usize,
            (sample_rate * 0.0083) as usize,
            (sample_rate * 0.0109) as usize,
        ];

        // Initialize comb filters
        for &delay in &comb_delays {
            self.comb_filters.push(CombFilter::new(delay, 0.84, 0.2));
        }

        // Initialize all-pass filters
        for &delay in &allpass_delays {
            self.all_pass_filters.push(AllPassFilter::new(delay, 0.5));
        }

        self.update_parameters();
    }

    fn update_parameters(&mut self) {
        let room_scale = self.room_size.value * 0.28 + 0.7;
        let damping = self.damping.value * 0.4;

        for comb in &mut self.comb_filters {
            comb.set_feedback(room_scale);
            comb.set_damping(damping);
        }
    }

    pub fn set_room_size(&mut self, size: f32) {
        self.room_size.set_value(size);
        self.update_parameters();
    }

    pub fn set_damping(&mut self, damping: f32) {
        self.damping.set_value(damping);
        self.update_parameters();
    }

    pub fn set_wet_level(&mut self, level: f32) {
        self.wet_level.set_value(level);
    }

    pub fn set_dry_level(&mut self, level: f32) {
        self.dry_level.set_value(level);
    }
}

impl EffectModule for Reverb {
    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        if !self.enabled {
            return;
        }

        let channels = self.channels;

        if channels == 1 {
            // Mono processing
            for (i, sample) in in_b.iter().enumerate() {
                let input = *sample;

                // Process through comb filters
                let mut comb_sum = 0.0;
                for comb in &mut self.comb_filters {
                    comb_sum += comb.process_internal(input);
                }

                // Process through all-pass filters
                let mut wet = comb_sum;
                for allpass in &mut self.all_pass_filters {
                    wet = allpass.process_internal(wet);
                }

                // Mix dry and wet signals
                out_b[i] = input * self.dry_level.value + wet * self.wet_level.value;
            }
        } else {
            // Stereo processing
            for i in (0..in_b.len()).step_by(2) {
                let left = in_b[i];
                let right = in_b[i + 1];

                // Mix to mono for reverb input
                let mono = (left + right) * 0.5;

                // Process through comb filters
                let mut comb_sum = 0.0;
                for comb in &mut self.comb_filters {
                    comb_sum += comb.process_internal(mono);
                }

                // Process through all-pass filters
                let mut wet = comb_sum;
                for allpass in &mut self.all_pass_filters {
                    wet = allpass.process_internal(wet);
                }

                // Create stereo wet signal
                let wet_left = wet * self.wet_level.value;
                let wet_right = wet * self.wet_level.value * self.width.value;

                // Mix with dry signal
                out_b[i] = left * self.dry_level.value + wet_left;
                out_b[i + 1] = right * self.dry_level.value + wet_right;
            }
        }
    }

    fn reset(&mut self) {
        for allpass in &mut self.all_pass_filters {
            allpass.reset();
        }
        for comb in &mut self.comb_filters {
            comb.reset();
        }
    }

    fn name(&self) -> &str {
        "reverb"
    }

}
