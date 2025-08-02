use fundsp::prelude::*;
use std::sync::{Arc, Mutex};

use crate::dsp::effect_trait::AudioEffect;

pub struct AudioProcessor {
    pitch_shifter: Arc<Mutex<PitchShifter>>,
    vibrato: Arc<Mutex<Box<dyn AudioUnit>>>,
}

impl AudioProcessor {
    pub fn new() -> Self {
        AudioProcessor {
            pitch_shifter: Arc::new(Mutex::new(PitchShifter::new(2048, 1.5))),
            vibrato: Arc::new(Mutex::new(Box::new(sine_hz::<f32>(4.0) * 0.02 + 1.0))),
        }
    }

    pub fn process(&self, data: &[f32]) -> Vec<f32> {
        let mut pitch_shifter = self.pitch_shifter.lock().unwrap();
        let mut vibrato = self.vibrato.lock().unwrap();
        anime_voice_process(data, &mut *pitch_shifter, &mut *vibrato)
    }
}

fn anime_voice_process(
    data: &[f32],
    pitch_shifter: &mut PitchShifter,
    vibrato: &mut Box<dyn AudioUnit>,
) -> Vec<f32> {
    data.iter().map(|&sample| {
        // Pitch shift
        let mut pitched = pitch_shifter.process(sample);
        
        // Dodaj wibrację
        let vibrato_amount = vibrato.get_mono();
        pitched *= vibrato_amount;
        
        // Formant enhancement - prosty high-shelf filter
        let enhanced = pitched * 1.2 + sample * 0.1;
        
        // Soft clipping dla "kawaii" charakteru
        let processed = enhanced.tanh() * 0.8;
        
        // Clamp do bezpiecznych wartości
        processed.clamp(-1.0, 1.0)
    }).collect()
}

pub struct AnimeVoice {}

impl AudioEffect for AnimeVoice {
    fn process(&mut self, input: &[f32], processor: &AudioProcessor) -> Vec<f32> {
        processor.process(input)
    }
}

struct PitchShifter {
    buffer: Vec<f32>,
    write_pos: usize,
    read_pos: f32,
    pitch_factor: f32,
    buffer_size: usize,
}

impl PitchShifter {
    fn new(buffer_size: usize, pitch_factor: f32) -> Self {
        Self {
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            read_pos: 0.0,
            pitch_factor,
            buffer_size,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        // Zapisz próbkę do buffera
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.buffer_size;

        // Czytaj z przesunięciem dla pitch shift
        let output = self.interpolated_read();

        // Przesuń pozycję odczytu
        self.read_pos += self.pitch_factor;
        if self.read_pos >= self.buffer_size as f32 {
            self.read_pos -= self.buffer_size as f32;
        }

        output
    }

    fn interpolated_read(&self) -> f32 {
        let index = self.read_pos as usize;
        let frac = self.read_pos - index as f32;

        let sample1 = self.buffer[index % self.buffer_size];
        let sample2 = self.buffer[(index + 1) % self.buffer_size];

        // Interpolacja liniowa
        sample1 * (1.0 - frac) + sample2 * frac
    }
}
