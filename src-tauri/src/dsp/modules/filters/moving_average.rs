pub struct MovingAverageFilter {
    buffer: Vec<f32>,
    index: usize,
    sum: f32,
}

impl MovingAverageFilter {
    pub fn new(window_size: usize) -> Self {
        Self {
            buffer: vec![0.0; window_size],
            index: 0,
            sum: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.sum -= self.buffer[self.index];
        self.buffer[self.index] = input;
        self.sum += input;
        self.index = (self.index + 1) % self.buffer.len();
        
        self.sum / self.buffer.len() as f32
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.sum = 0.0;
        self.index = 0;
    }
}