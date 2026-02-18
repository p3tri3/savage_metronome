// Defines the structure for a savable metronome preset.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MetronomePreset {
    pub bpm: f32,
    pub volume: f32,
    pub pitch_hz: f32,
    pub beep_duration: f32,
    pub visual_enabled: bool,
    pub tuning: Tuning,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tuning {
    pub reference_pitch: f32,
    pub octave: u8,
    pub note_index: usize,
}

impl Default for MetronomePreset {
    fn default() -> Self {
        let tuning = Tuning {
            reference_pitch: 440.0,
            octave: 6,
            note_index: 0, // C
        };
        Self {
            bpm: 100.0,
            volume: 0.5,
            pitch_hz: calculate_pitch(&tuning),
            beep_duration: 0.005,
            visual_enabled: true,
            tuning,
        }
    }
}

pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];

/// Compute the frequency (Hz) for a given tuning using the equal-temperament formula.
///
/// MIDI note number: `n = (octave + 1) * 12 + note_index`
/// where octave 4, note A (index 9) → n = 69, the standard A4 reference pitch.
/// Frequency: `f = reference_pitch * 2^((n - 69) / 12)`
pub fn calculate_pitch(tuning: &Tuning) -> f32 {
    let n = (tuning.octave as i32 + 1) * 12 + tuning.note_index as i32;
    tuning.reference_pitch * 2.0_f32.powf((n - 69) as f32 / 12.0)
}

impl MetronomePreset {
    /// Clamp all fields to safe ranges after deserialization from untrusted input.
    pub fn sanitize(mut self) -> Self {
        self.bpm = if self.bpm.is_finite() {
            self.bpm.clamp(20.0, 300.0)
        } else {
            100.0
        };
        self.volume = if self.volume.is_finite() {
            self.volume.clamp(0.0, 1.0)
        } else {
            0.5
        };
        self.beep_duration = if self.beep_duration.is_finite() && self.beep_duration > 0.0 {
            self.beep_duration
        } else {
            0.005
        };
        self.tuning.note_index = self.tuning.note_index.min(NOTE_NAMES.len() - 1);
        self.tuning.reference_pitch =
            if self.tuning.reference_pitch.is_finite() && self.tuning.reference_pitch > 0.0 {
                self.tuning.reference_pitch
            } else {
                440.0
            };
        // Recalculate pitch_hz from the now-sanitized tuning to ensure consistency.
        self.pitch_hz = calculate_pitch(&self.tuning);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pitch() {
        let tuning = Tuning {
            reference_pitch: 440.0,
            octave: 4,
            note_index: 9, // A
        };
        assert!((calculate_pitch(&tuning) - 440.0).abs() < 0.001);

        let tuning_c4 = Tuning {
            reference_pitch: 440.0,
            octave: 4,
            note_index: 0, // C
        };
        assert!((calculate_pitch(&tuning_c4) - 261.625).abs() < 0.01);
    }
}
