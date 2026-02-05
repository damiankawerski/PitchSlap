#[derive(Debug, Clone)]
pub struct NoteEvent {
    /// Note name (C, D, E, F, G, A, B)
    pub note: String,
    /// Octave (0-8)
    pub octave: u8,
    /// Pitch in Hz
    pub frequency: f32,
    /// Note duration in beats
    pub duration: f32,
    /// Note velocity (0.0-1.0)
    pub velocity: f32,
    /// Vibrato intensity (0.0-1.0)
    pub vibrato: f32,
    /// Lyric text for this note
    pub lyric: Option<String>,
    /// Phoneme sequence for this note
    pub phonemes: Vec<String>,
    /// Timing offset in seconds
    pub timing_offset: f32,
    /// Breath before note (0.0-1.0)
    pub breath_before: f32,
    /// Legato connection to next note
    pub legato: bool,
}

impl NoteEvent {
    pub fn new(note: String, octave: u8, duration: f32, velocity: f32) -> Self {
        Self {
            frequency: Self::note_to_frequency(&note, octave),
            note,
            octave,
            duration,
            velocity,
            vibrato: 0.5,
            lyric: None,
            phonemes: Vec::new(),
            timing_offset: 0.0,
            breath_before: 0.0,
            legato: false,
        }
    }

    pub fn note_to_frequency(note: &str, octave: u8) -> f32 {
        let base_frequencies = [
            ("C", 16.35),
            ("C#", 17.32),
            ("Db", 17.32),
            ("D", 18.35),
            ("D#", 19.45),
            ("Eb", 19.45),
            ("E", 20.60),
            ("F", 21.83),
            ("F#", 23.12),
            ("Gb", 23.12),
            ("G", 24.50),
            ("G#", 25.96),
            ("Ab", 25.96),
            ("A", 27.50),
            ("A#", 29.14),
            ("Bb", 29.14),
            ("B", 30.87),
        ];

        let base_freq = base_frequencies
            .iter()
            .find(|(n, _)| n == &note)
            .map(|(_, f)| f)
            .copied()
            .unwrap_or(27.50); // Default to A0

        base_freq * 2.0_f32.powi(octave as i32)
    }

}
