use crate::dsp::traits::EffectModule;
use super::pitch_shifter::PitchShifter;
use std::collections::VecDeque;
use pitch_detection::detector::PitchDetector;
use pitch_detection::detector::yin::YINDetector;
use crate::dsp::modules::utils::note_event::NoteEvent;

pub struct AutoTune {
    pitch_shifter: PitchShifter,
    sample_rate: f32,
    detection_buffer: VecDeque<f32>,
    detection_window_size: usize,
    
    // Parametry autotune
    correction_speed: f32, // 0.0 do 1.0
    current_shift: f32,
    
    // Logic for sustaining pitch during unvoiced segments
    last_valid_shift: f32,
    sustain_counter: usize,
    max_sustain: usize,

    target_note: Option<NoteEvent>,
    last_detected_note: Option<NoteEvent>,
    scale: Scale,
}

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

impl AutoTune {
    pub fn new(sample_rate: f32) -> Self {
        // Konfiguracja PitchShifter: zmniejszmy latencję ( np. 30ms ), oversampling 8x dla lepszej jakości przy dużych zmianach
        let pitch_shifter = PitchShifter::new(30, sample_rate as usize, 0.0, 8);
        
        // Mniejsze okno detekcji dla szybszej reakcji (np. 1536 próbek ~35ms)
        let detection_window_size = 1536;

        Self {
            pitch_shifter,
            sample_rate,
            detection_buffer: VecDeque::with_capacity(detection_window_size),
            detection_window_size,
            correction_speed: 0.95, // Bardzo agresywna korekcja
            current_shift: 0.0,
            last_valid_shift: 0.0,
            sustain_counter: 0,
            max_sustain: (sample_rate * 0.2) as usize, // 200ms sustain

            target_note: None,
            last_detected_note: None,
            scale: Scale::CMajor,
        }
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    pub fn set_target_note(&mut self, note: Option<NoteEvent>) {
        self.target_note = note;
    }

    pub fn last_detected_note(&self) -> Option<NoteEvent> {
        self.last_detected_note.clone()
    }

    pub fn set_correction_speed(&mut self, speed: f32) {
        self.correction_speed = speed.max(0.0).min(1.0);
    }

    fn detect_pitch(&self) -> Option<f32> {
        if self.detection_buffer.len() < self.detection_window_size {
            return None;
        }

        // Kopiowanie do wektora dla łatwiejszego dostępu i konwersja na f64
        let buffer: Vec<f64> = self.detection_buffer.iter().map(|&s| s as f64).collect();
        
        // YIN z pitch-detection
        let mut yin = YINDetector::new(self.detection_window_size, self.detection_window_size / 2);
        
        // Agresywne progi dla mowy (niższy power_threshold, niższa clarity)
        match yin.get_pitch(&buffer, self.sample_rate as usize, 0.05, 0.3) {
            Some(pitch) => Some(pitch.frequency as f32),
            None => None
        }
    }

    fn calculate_shift(&self, detected_freq: f32) -> f32 {
        if detected_freq < 50.0 { // Ignoruj dziwne niskie częstotliwości
            return 0.0;
        }

        // 1. Znajdź midi note number
        let note_num = 12.0 * (detected_freq / 440.0).log2() + 69.0;
        
        // 2. Snap to configured scale
        let note_in_octave = (note_num.round() as i32 % 12 + 12) % 12;

        let is_in_scale = self.scale_contains(note_in_octave);
        
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
                 if self.scale_contains(cand_oct) {
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
        
        // 3. Oblicz różnicę w półtonach do note_num (który wywodzi się z detected_freq)
        //    Wzór: target - current
        //    Ale detected_freq odpowiada dokładnie note_num (float).
        //    Chcemy przesunąć o (target_note - note_num)
        
        let diff = target_note - note_num;
        diff
    }

fn scale_contains(&self, note_in_octave: i32) -> bool {
    match self.scale {
        // C Major: C D E F G A B
        Scale::CMajor => matches!(note_in_octave, 0 | 2 | 4 | 5 | 7 | 9 | 11),
        
        // A Major: A B C# D E F# G#
        Scale::AMajor => matches!(note_in_octave, 9 | 11 | 1 | 2 | 4 | 6 | 8),
        
        // G Major: G A B C D E F#
        Scale::GMajor => matches!(note_in_octave, 7 | 9 | 11 | 0 | 2 | 4 | 6),
        
        // D Major: D E F# G A B C#
        Scale::DMajor => matches!(note_in_octave, 2 | 4 | 6 | 7 | 9 | 11 | 1),
        
        // E Major: E F# G# A B C# D#
        Scale::EMajor => matches!(note_in_octave, 4 | 6 | 8 | 9 | 11 | 1 | 3),
        
        // F Major: F G A Bb C D E
        Scale::FMajor => matches!(note_in_octave, 5 | 7 | 9 | 10 | 0 | 2 | 4),
        
        // G Minor: G A Bb C D Eb F
        Scale::GMinor => matches!(note_in_octave, 7 | 9 | 10 | 0 | 2 | 3 | 5),
        
        // D Minor: D E F G A Bb C
        Scale::DMinor => matches!(note_in_octave, 2 | 4 | 5 | 7 | 9 | 10 | 0),
        
        // A Minor: A B C D E F G
        Scale::AMinor => matches!(note_in_octave, 9 | 11 | 0 | 2 | 4 | 5 | 7),
        
        // E Minor: E F# G A B C D
        Scale::EMinor => matches!(note_in_octave, 4 | 6 | 7 | 9 | 11 | 0 | 2),
    }
}
}

impl EffectModule for AutoTune {
    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        // 1. Zaktualizuj bufor detekcji
        for &sample in in_b {
            if self.detection_buffer.len() >= self.detection_window_size {
                self.detection_buffer.pop_front();
            }
            self.detection_buffer.push_back(sample);
        }

        // 2. Detekcja
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

        // 3. Aplikuj shift
        self.pitch_shifter.set_shift(self.current_shift);
        
        // 4. Przetwarzaj audio przez PitchShifter
        self.pitch_shifter.process(in_b, out_b);
    }

    fn reset(&mut self) {
        self.detection_buffer.clear();
        self.current_shift = 0.0;
        self.last_valid_shift = 0.0;
        self.sustain_counter = 0;
        self.pitch_shifter.reset();
    }

    fn name(&self) -> &str {
        "auto_tune"
    }
}