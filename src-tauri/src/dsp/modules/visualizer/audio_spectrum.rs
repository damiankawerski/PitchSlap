use serde::Serialize;

/// For real time visualization
#[derive(Clone, Debug, Serialize)]
pub struct AudioFrame {
    // --- podstawowe metryki do wizualizacji ---
    pub rms: f32,                 // bieżąca energia / głośność
    pub pitch: f32,       // wysokość głosu w Hz (jeśli dostępna)
    pub spectrum: Vec<f32>, // FFT snapshot, np. 16-32 pasm znormalizowanych 0-1
    pub frequencies: Vec<f32>, // odpowiadające częstotliwości dla pasm FFT
  
    // --- identyfikatory / synchronizacja ---
    pub timestamp: u64,           // czas w sekundach od startu nagrania
}

impl AudioFrame {
    pub fn new(
        rms: f32,
        pitch: f32,
        spectrum: Vec<f32>,
        frequencies: Vec<f32>,
        timestamp: u64,
    ) -> Self {
        Self {
            rms,
            pitch,
            spectrum,
            frequencies,
            timestamp,
        }
    }
}