
use fundsp::prelude::*;

// =============================================================================
// GŁÓWNY AUDIO PROCESSOR - Zawiera wszystkie narzędzia DSP
// =============================================================================

pub struct AudioProcessor {
    // Pitch shifting
    pitch_shifter: PitchShifter,
    
    // Oscylatory i modulatory
    ring_modulator: Box<dyn AudioUnit>,
    vibrato: Box<dyn AudioUnit>,
    lfo_slow: Box<dyn AudioUnit>,
    lfo_fast: Box<dyn AudioUnit>,
    
    // Filtry
    lowpass_filter: Box<dyn AudioUnit>,
    highpass_filter: Box<dyn AudioUnit>,
    bandpass_filter: Box<dyn AudioUnit>,
    resonator: Box<dyn AudioUnit>,
    
    // Efekty czasowe
    delay_lines: Vec<DelayLine>,
    modulated_delays: Vec<ModulatedDelayLine>,
    all_pass_filters: Vec<AllPassFilter>,
    
    // Narzędzia do zniekształceń
    bit_crusher: BitCrusher,
    
    // Formant processing
    formant_shifter: FormantShifter,
    
    // Noise generator dla breathiness
    noise_gen: Box<dyn AudioUnit>,
    
    // Sample rate do obliczeń
    sample_rate: f32,
    
    // Bufory robocze
    temp_buffer: Vec<f32>,
}

impl AudioProcessor {
    pub fn new(sample_rate: f32) -> Self {
        // Inicjalizuj delay lines dla reverb
        let delay_times = vec![1323, 1557, 1789, 2011]; // samples
        let delay_lines: Vec<DelayLine> = delay_times.iter()
            .map(|&samples| DelayLine::new(samples))
            .collect();
            
        // All-pass filters dla reverb
        let allpass_times = vec![221, 441, 663];
        let all_pass_filters: Vec<AllPassFilter> = allpass_times.iter()
            .map(|&samples| AllPassFilter::new(samples, 0.7))
            .collect();
            
        // Modulated delays dla chorus
        let modulated_delays = vec![
            ModulatedDelayLine::new(0.02, 0.005, 0.5, sample_rate),
            ModulatedDelayLine::new(0.025, 0.007, 0.7, sample_rate),
            ModulatedDelayLine::new(0.03, 0.006, 0.3, sample_rate),
        ];
        
        Self {
            pitch_shifter: PitchShifter::new(2048, 1.0),
            
            // Oscylatory
            ring_modulator: Box::new(sine_hz::<f32>(120.0)),
            vibrato: Box::new(sine_hz::<f32>(4.0) * 0.02 + 1.0),
            lfo_slow: Box::new(sine_hz::<f32>(0.5)),
            lfo_fast: Box::new(sine_hz::<f32>(8.0)),
            
            // Filtry
            lowpass_filter: Box::new(lowpass_hz::<f32>(3000.0, 1.0)),
            highpass_filter: Box::new(highpass_hz::<f32>(100.0, 1.0)),
            bandpass_filter: Box::new(bandpass_hz::<f32>(1000.0, 50.0)),
            resonator: Box::new(resonator_hz::<f32>(120.0, 0.8)),
            
            // Efekty czasowe
            delay_lines,
            modulated_delays,
            all_pass_filters,
            
            // Zniekształcenia
            bit_crusher: BitCrusher::new(8),
            
            // Formant processing
            formant_shifter: FormantShifter::new(),
            
            // Noise
            noise_gen: Box::new(white() * 0.05),
            
            sample_rate,
            temp_buffer: Vec::with_capacity(1024),
        }
    }
    
    // =============================================================================
    // METODY PROCESSINGOWE
    // =============================================================================
    
    pub fn set_pitch(&mut self, factor: f32) {
        self.pitch_shifter.set_pitch_factor(factor);
    }
    
    pub fn process_pitch_shift(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.pitch_shifter.process(sample)
        }).collect()
    }
    
    pub fn process_ring_modulation(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            let mod_signal = self.ring_modulator.get_mono();
            sample * (0.5 + 0.5 * mod_signal)
        }).collect()
    }
    
    pub fn process_vibrato(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            let vibrato_amount = self.vibrato.get_mono();
            sample * vibrato_amount
        }).collect()
    }
    
    pub fn process_lowpass(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.lowpass_filter.filter_mono(sample)
        }).collect()
    }
    
    pub fn process_highpass(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.highpass_filter.filter_mono(sample)
        }).collect()
    }
    
    pub fn process_resonator(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.resonator.filter_mono(sample)
        }).collect()
    }
    
    pub fn process_bit_crush(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.bit_crusher.process(sample)
        }).collect()
    }
    
    pub fn process_reverb(&mut self, input: &[f32], wet_level: f32) -> Vec<f32> {
        input.iter().map(|&sample| {
            let mut reverb_sum = 0.0;
            
            // Delay lines
            for delay_line in &mut self.delay_lines {
                let delayed = delay_line.process(sample + reverb_sum * 0.3);
                reverb_sum += delayed * 0.25;
            }
            
            // All-pass filters
            for all_pass in &mut self.all_pass_filters {
                reverb_sum = all_pass.process(reverb_sum);
            }
            
            // Mix dry/wet
            sample * (1.0 - wet_level) + reverb_sum * wet_level
        }).collect()
    }
    
    pub fn process_chorus(&mut self, input: &[f32], wet_level: f32) -> Vec<f32> {
        input.iter().map(|&sample| {
            let mut chorus_sum = 0.0;
            
            for delay_line in &mut self.modulated_delays {
                chorus_sum += delay_line.process(sample);
            }
            
            sample * (1.0 - wet_level) + chorus_sum * wet_level / 3.0
        }).collect()
    }
    
    pub fn process_formant_shift(&mut self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&sample| {
            self.formant_shifter.process(sample)
        }).collect()
    }
    
    pub fn add_breathiness(&mut self, input: &[f32], amount: f32) -> Vec<f32> {
        input.iter().map(|&sample| {
            let breath = self.noise_gen.get_mono();
            sample + breath * sample.abs() * amount
        }).collect()
    }
    
    pub fn soft_clip(&self, input: &[f32], amount: f32) -> Vec<f32> {
        input.iter().map(|&sample| {
            (sample * amount).tanh() * (1.0 / amount.tanh())
        }).collect()
    }
    
    pub fn hard_clip(&self, input: &[f32], threshold: f32) -> Vec<f32> {
        input.iter().map(|&sample| {
            sample.clamp(-threshold, threshold)
        }).collect()
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
    
    fn set_pitch_factor(&mut self, factor: f32) {
        self.pitch_factor = factor;
    }

    fn process(&mut self, input: f32) -> f32 {
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.buffer_size;

        let output = self.interpolated_read();

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

        sample1 * (1.0 - frac) + sample2 * frac
    }
}

struct DelayLine {
    buffer: Vec<f32>,
    index: usize,
}

impl DelayLine {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            index: 0,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];
        self.buffer[self.index] = input;
        self.index = (self.index + 1) % self.buffer.len();
        output
    }
}

struct ModulatedDelayLine {
    buffer: Vec<f32>,
    write_index: usize,
    base_delay: f32,
    mod_depth: f32,
    mod_rate: f32,
    phase: f32,
    sample_rate: f32,
}

impl ModulatedDelayLine {
    fn new(base_delay: f32, mod_depth: f32, mod_rate: f32, sample_rate: f32) -> Self {
        let buffer_size = ((base_delay + mod_depth) * sample_rate) as usize + 100;
        Self {
            buffer: vec![0.0; buffer_size],
            write_index: 0,
            base_delay: base_delay * sample_rate,
            mod_depth: mod_depth * sample_rate,
            mod_rate,
            phase: 0.0,
            sample_rate,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        self.buffer[self.write_index] = input;
        
        let mod_delay = self.base_delay + self.mod_depth * (self.phase * 2.0 * std::f32::consts::PI).sin();
        let read_index = (self.write_index as f32 - mod_delay) % self.buffer.len() as f32;
        
        let output = self.interpolated_read(read_index);
        
        self.write_index = (self.write_index + 1) % self.buffer.len();
        self.phase += self.mod_rate / self.sample_rate;
        if self.phase > 1.0 { self.phase -= 1.0; }
        
        output
    }
    
    fn interpolated_read(&self, pos: f32) -> f32 {
        let index = pos as usize % self.buffer.len();
        let next_index = (index + 1) % self.buffer.len();
        let frac = pos - pos.floor();
        
        self.buffer[index] * (1.0 - frac) + self.buffer[next_index] * frac
    }
}

struct AllPassFilter {
    buffer: Vec<f32>,
    index: usize,
    gain: f32,
}

impl AllPassFilter {
    fn new(size: usize, gain: f32) -> Self {
        Self {
            buffer: vec![0.0; size],
            index: 0,
            gain,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.index];
        let output = -input * self.gain + delayed;
        self.buffer[self.index] = input + delayed * self.gain;
        self.index = (self.index + 1) % self.buffer.len();
        output
    }
}

struct BitCrusher {
    bits: f32,
}

impl BitCrusher {
    fn new(bits: u8) -> Self {
        Self { bits: bits as f32 }
    }
    
    fn process(&self, input: f32) -> f32 {
        let max_val = 2_f32.powf(self.bits - 1.0);
        (input * max_val).round() / max_val
    }
}

struct FormantShifter {
    delay_buffer: Vec<f32>,
    index: usize,
}

impl FormantShifter {
    fn new() -> Self {
        Self {
            delay_buffer: vec![0.0; 64],
            index: 0,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.delay_buffer[self.index];
        self.delay_buffer[self.index] = input;
        self.index = (self.index + 1) % self.delay_buffer.len();
        
        input * 0.7 + delayed * 0.3
    }
}