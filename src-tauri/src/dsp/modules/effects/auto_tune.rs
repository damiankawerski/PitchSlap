use super::pitch_shifter::PitchShifter;
use crate::dsp::modules::yin_detector::detector::PitchDetector;
use crate::dsp::modules::yin_detector::detector::yin::YINDetector;
use crate::dsp::traits::EffectModule;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    CMajor,
    AMajor,
    GMajor,
    DMajor,
    EMajor,
    FMajor,
    GMinor,
    DMinor,
    AMinor,
    EMinor,
}

impl Scale {
    pub fn get_notes(&self) -> Vec<u8> {
        match self {
            Scale::CMajor => vec![0, 2, 4, 5, 7, 9, 11], // C D E F G A B
            Scale::AMajor => vec![9, 11, 0, 2, 4, 5, 7], // A B C# D E F# G#
            Scale::GMajor => vec![7, 9, 11, 0, 2, 4, 5], // G A B C D E F#
            Scale::DMajor => vec![2, 4, 6, 7, 9, 11, 0], // D E F# G A B C#
            Scale::EMajor => vec![4, 6, 8, 9, 11, 1, 3], // E F# G# A B C# D#
            Scale::FMajor => vec![5, 7, 9, 10, 0, 2, 4], // F G A Bb C D E
            Scale::GMinor => vec![7, 9, 10, 0, 2, 3, 5], // G A Bb C D Eb F
            Scale::DMinor => vec![2, 4, 5, 7, 9, 10, 0], // D E F G A Bb C
            Scale::AMinor => vec![9, 11, 0, 2, 4, 5, 7], // A B C D E F G
            Scale::EMinor => vec![4, 6, 7, 9, 11, 0, 2], // E F# G A B C D
        }
    }
}

pub struct AutoTune {
    sample_rate: f32,
    pitch_shifter: PitchShifter,
    yin_detector: YINDetector<f32>,
    detection_buffer: VecDeque<f32>,

    // Parametry autotune
    correction_speed: f32, // 0.0 do 1.0
    detection_window_size: usize,
    power_threshold: f32,
    clarity_threshold: f32,

    // Logic for sustaining pitch during unvoiced segments
    last_valid_shift: f32,
    sustain_counter: usize,
    max_sustain: usize,
    current_shift: f32,

    scale: Scale,
}

impl AutoTune {
    pub fn new(sample_rate: f32) -> Self {
        let pitch_shifter = PitchShifter::new(30, sample_rate as usize, 0.0, 8);

        let detection_window_size = 1536;

        Self {
            pitch_shifter,
            sample_rate,
            detection_buffer: VecDeque::with_capacity(detection_window_size),
            detection_window_size,
            correction_speed: 0.95,
            current_shift: 0.0,
            last_valid_shift: 0.0,
            sustain_counter: 0,
            max_sustain: (sample_rate * 0.2) as usize,
            yin_detector: YINDetector::new(detection_window_size, detection_window_size / 2),
            power_threshold: 0.05,
            clarity_threshold: 0.3,
            scale: Scale::CMajor,
        }
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    pub fn set_correction_speed(&mut self, speed: f32) {
        self.correction_speed = speed.clamp(0.0, 1.0);
    }

    pub fn set_max_sustain(&mut self, sustain_time_ms: f32) {
        self.max_sustain = (self.sample_rate * (sustain_time_ms / 1000.0)) as usize;
    }

    pub fn set_power_threshold(&mut self, threshold: f32) {
        self.power_threshold = threshold.clamp(0.0, 1.0);
    }

    pub fn set_clarity_threshold(&mut self, threshold: f32) {
        self.clarity_threshold = threshold.clamp(0.0, 1.0);
    }

    pub fn set_detection_window_size(&mut self, size: usize) {
        self.detection_window_size = size;
        self.detection_buffer = VecDeque::with_capacity(size);
        self.yin_detector = YINDetector::new(size, size / 2);
    }

    fn detect_pitch(&mut self) -> Option<f32> {
        if self.detection_buffer.len() < self.detection_window_size {
            return None;
        }

        let buffer: Vec<f32> = self.detection_buffer.iter().map(|&s| s as f32).collect();

        match self.yin_detector.get_pitch(
            &buffer,
            self.sample_rate as usize,
            self.power_threshold,
            self.clarity_threshold,
        ) {
            Some(pitch) => Some(pitch.frequency as f32),
            None => None,
        }
    }

    fn calculate_shift(&self, detected_freq: f32) -> f32 {
        if detected_freq < 50.0 {
            return 0.0;
        }

        // Find MIDI note number
        let note_num = 12.0 * (detected_freq / 440.0).log2() + 69.0;

        // Snap to configured scale
        let note_in_octave = (note_num.round() as i32 % 12 + 12) % 12;

        let is_in_scale = self.scale.get_notes().contains(&(note_in_octave as u8));

        let target_note = if is_in_scale {
            note_num.round()
        } else {
            // Jeśli nie w skali, znajdź najbliższą.
            // Prostym sposobem jest sprawdzenie +/- 1
            let check_offsets = [-1.0, 1.0];
            let mut best_n = note_num.round();
            let mut min_dist = 999.0;

            for offset in check_offsets {
                let candidate = note_num.round() + offset;
                let cand_oct = (candidate as i32 % 12 + 12) % 12;
                if self.scale.get_notes().contains(&(cand_oct as u8)) {
                    // Check 'true' distance from float note_num
                    let dist = (candidate - note_num).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        best_n = candidate;
                    }
                }
            }
            best_n
        };

        let diff = target_note - note_num;
        diff
    }

    fn process_internal(&mut self, input: &[f32], output: &mut [f32]) {
        // Update detection buffer 
        for &sample in input {
            if self.detection_buffer.len() >= self.detection_window_size {
                self.detection_buffer.pop_front();
            }
            self.detection_buffer.push_back(sample);
        }

        // Detect pitch and calculate shift
        if let Some(freq) = self.detect_pitch() {
            
            let target_shift = self.calculate_shift(freq);
            
            // Hard Tune: Szybka korekcja
            self.current_shift = self.current_shift + (target_shift - self.current_shift) * self.correction_speed;
            
            // Update sustain
            self.last_valid_shift = self.current_shift;
            self.sustain_counter = self.max_sustain;
        } else {
            // Sustain logic
            if self.sustain_counter > 0 {
                self.sustain_counter -= 1;
                // Keep using last pitch
                self.current_shift = self.last_valid_shift;
            } else {
                // Fade out to 0 shift
                self.current_shift = self.current_shift * 0.9;
            }
        }

        // Apply shift 
        self.pitch_shifter.set_shift(self.current_shift);
        
        // Process audio through PitchShifter
        self.pitch_shifter.process(input, output);
        }
}


impl EffectModule for AutoTune {
    fn name(&self) -> &str {
        "AutoTune"
    }

    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        self.process_internal(in_b, out_b);
    }

    fn reset(&mut self) {
        self.detection_buffer.clear();
        self.current_shift = 0.0;
        self.last_valid_shift = 0.0;
        self.sustain_counter = 0;
        self.pitch_shifter.reset();
    }
}