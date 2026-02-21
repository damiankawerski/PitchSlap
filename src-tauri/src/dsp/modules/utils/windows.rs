pub fn hanning(size: usize) -> Vec<f32> {
    (0..size)
        .map(|n| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * n as f32 / (size as f32 - 1.0)).cos())
        .collect()
}

pub fn apply_hanning_window(buffer: &mut [f32]) {
    let window: Vec<f32> = hanning(buffer.len());
    for (sample, &window_val) in buffer.iter_mut().zip(window.iter()) {
        *sample *= window_val;
    }
}

pub fn apply_hanning_window_copy(buffer: &[f32]) -> Vec<f32> {
    let window: Vec<f32> = hanning(buffer.len());
    buffer.iter()
        .zip(window.iter())
        .map(|(sample, window_val)| sample * window_val)
        .collect()
}