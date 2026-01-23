use crate::dsp::traits::EffectModule;

/// Reverb effect using FunDSP's built-in reverb algorithms
pub struct FundspReverb {
    /// Delay lines for reverb
    delay_lines: Vec<Vec<f32>>,
    /// Delay line positions
    positions: Vec<usize>,
    /// Sample rate
    sample_rate: f32,
    /// Reverb room size (0.0 to 1.0)
    room_size: f32,
    /// Reverb time in seconds
    time: f32,
    /// Damping factor (0.0 to 1.0)
    damping: f32,
    /// Dry/wet mix (0.0 = fully dry, 1.0 = fully wet)
    mix: f32,
    /// Low-pass filter state for damping
    lowpass_state: f32,
}

impl FundspReverb {
    /// Create a new reverb effect
    /// 
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    /// * `room_size` - Room size (0.0 to 1.0, default 0.5)
    /// * `time` - Reverb time in seconds (default 2.0)
    /// * `damping` - High frequency damping (0.0 to 1.0, default 0.5)
    /// * `mix` - Dry/wet mix (0.0 to 1.0, default 0.3)
    pub fn new(
        sample_rate: f32,
        room_size: f32,
        time: f32,
        damping: f32,
        mix: f32,
    ) -> Self {
        let room_size = room_size.clamp(0.0, 1.0);
        let time = time.max(0.1);
        let damping = damping.clamp(0.0, 1.0);
        
        // Create multiple delay lines with prime number lengths for better diffusion
        // Longer delays for more spacious reverb
        let delay_times = [
            0.0297, 0.0371, 0.0411, 0.0437, 0.0527, 0.0617,  // Early reflections
            0.0890, 0.0973, 0.1051, 0.1129,                    // Mid reflections
            0.0050, 0.0017, 0.0103, 0.0089, 0.0067, 0.0121,    // Short delays for density
        ];
        
        let mut delay_lines = Vec::new();
        for &delay_time in &delay_times {
            let delay_samples = (delay_time * sample_rate * (1.0 + room_size)) as usize;
            delay_lines.push(vec![0.0; std::cmp::max(delay_samples, 1)]);
        }
        
        let positions = vec![0; delay_lines.len()];
        
        Self {
            delay_lines,
            positions,
            sample_rate,
            room_size,
            time,
            damping,
            mix: mix.clamp(0.0, 1.0),
            lowpass_state: 0.0,
        }
    }
    
    /// Update reverb parameters
    pub fn set_room_size(&mut self, room_size: f32) {
        self.room_size = room_size.clamp(0.0, 1.0);
        self.rebuild_delays();
    }
    
    pub fn set_time(&mut self, time: f32) {
        self.time = time.max(0.1);
    }
    
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.0, 1.0);
    }
    
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
    
    /// Rebuild delay lines with new room size
    fn rebuild_delays(&mut self) {
        let delay_times = [
            0.0297, 0.0371, 0.0411, 0.0437, 0.0527, 0.0617,  // Early reflections
            0.0890, 0.0973, 0.1051, 0.1129,                    // Mid reflections
            0.0050, 0.0017, 0.0103, 0.0089, 0.0067, 0.0121,    // Short delays for density
        ];
        
        self.delay_lines.clear();
        for &delay_time in &delay_times {
            let delay_samples = (delay_time * self.sample_rate * (1.0 + self.room_size)) as usize;
            self.delay_lines.push(vec![0.0; std::cmp::max(delay_samples, 1)]);
        }
        self.positions = vec![0; self.delay_lines.len()];
        self.lowpass_state = 0.0;
    }
}

impl EffectModule for FundspReverb {
    fn process(&mut self, in_b: &[f32], out_b: &mut [f32]) {
        let num_samples = std::cmp::min(in_b.len(), out_b.len());
        
        if num_samples == 0 {
            return;
        }
        
        // Stronger feedback decay factor based on reverb time
        // Use more aggressive decay for longer tail
        let decay = (-1.5 / (self.time * self.sample_rate)).exp();
        
        // Process samples through delay lines
        for i in 0..num_samples {
            let input = in_b[i];
            
            // Accumulate output from all delay lines
            let mut wet_output = 0.0;
            
            for (idx, delay_line) in self.delay_lines.iter_mut().enumerate() {
                let pos = self.positions[idx];
                let delay_len = delay_line.len();
                
                // Read from delay line
                let delayed = delay_line[pos];
                
                // Apply damping with low-pass filter
                self.lowpass_state = self.lowpass_state * self.damping + delayed * (1.0 - self.damping);
                let filtered = self.lowpass_state;
                
                // Write to delay line with stronger feedback
                delay_line[pos] = input * 0.5 + filtered * decay * 0.85;
                
                // Update position
                self.positions[idx] = (pos + 1) % delay_len;
                
                // Accumulate output
                wet_output += filtered;
            }
            
            // Average and boost output for more presence
            wet_output /= self.delay_lines.len() as f32;
            wet_output *= 2.5;  // Boost wet signal
            
            // Mix dry and wet signals
            out_b[i] = input * (1.0 - self.mix) + wet_output * self.mix;
        }
    }
    
    fn reset_state(&mut self) {
        for delay_line in &mut self.delay_lines {
            delay_line.fill(0.0);
        }
        self.positions.fill(0);
        self.lowpass_state = 0.0;
    }
}

impl Default for FundspReverb {
    fn default() -> Self {
        Self::new(
            48000.0,  // sample_rate
            0.8,      // room_size - larger room
            3.5,      // time - longer decay
            0.3,      // damping - less damping for brighter sound
            0.6,      // mix - more wet signal
        )
    }
}

