use super::modules::time_domain::pitch_shifter::PitchShifter;
use super::modules::time_domain::formant_shifter::FormantShifter;

pub struct AudioProcessor {
    input_accumulator: Vec<f32>,
    buffer: Vec<f32>,
    pitch_shifter: PitchShifter,
    formant_shifter: FormantShifter,
    frame_size: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: usize) -> Self {
        let frame_size = 2048;
        Self {
            input_accumulator: Vec::with_capacity(frame_size),
            buffer: vec![0.0; frame_size],
            pitch_shifter: PitchShifter::new(10, sample_rate),
            formant_shifter: FormantShifter::new(10, sample_rate),
            frame_size,
        }
    }

    // Testowy process_pitch_shift
    pub fn process_pitch_shift(&mut self, input: &[f32]) -> Option<&[f32]> {
        self.input_accumulator.extend_from_slice(input);

        if self.input_accumulator.len() >= self.frame_size {
            let frame = self.input_accumulator.drain(..self.frame_size).collect::<Vec<_>>();
            self.pitch_shifter
                .shift_pitch(2, -4.0, &frame, &mut self.buffer);

            Some(&self.buffer)
        } else {
            None // jeszcze za mało próbek — czekamy
        }
    }

    pub fn process_formant_shift(&mut self, input: &[f32]) -> Option<&[f32]> {
        self.input_accumulator.extend_from_slice(input);

        if self.input_accumulator.len() >= self.frame_size {
            let frame = self.input_accumulator.drain(..self.frame_size).collect::<Vec<_>>();
            self.formant_shifter
                .shift_formants(2.0, &frame, &mut self.buffer);

            Some(&self.buffer)
        } else {
            None // jeszcze za mało próbek — czekamy
        }
    }
}
