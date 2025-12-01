use super::modules::time_domain::pitch_shifter::PitchShifter;
use super::modules::time_domain::formant_shifter::FormantShifter;
use super::modules::fft::audio_spectrum::FFTProcessor;

pub struct AudioProcessor {
    fft_processor: FFTProcessor,
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
            fft_processor: FFTProcessor::new(sample_rate, 480, 60),
            frame_size,
        }
    }

    pub fn process_and_send_fft(&mut self, input: &[f32], app_handle: &tauri::AppHandle) {
        self.input_accumulator.extend_from_slice(input);

        let processed = self.input_accumulator.drain(..self.frame_size.min(self.input_accumulator.len())).collect::<Vec<_>>();

        if let Err(e) = self.fft_processor.process_and_send(app_handle, &processed) {
            eprintln!("FFT processing error: {}", e);
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
