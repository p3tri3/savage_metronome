// Contains the core state and logic of the metronome.
use crate::presets::preset::{MetronomePreset, Tuning};
use std::time::Instant;

pub struct Metronome {
    pub bpm: f32,
    pub volume: f32,
    pub pitch_hz: f32,
    pub beep_duration: f32,
    pub is_running: bool,
    pub visual_enabled: bool,
    pub last_beat: Option<Instant>,
    pub tuning: Tuning,
}

impl From<MetronomePreset> for Metronome {
    fn from(preset: MetronomePreset) -> Self {
        Self {
            bpm: preset.bpm,
            volume: preset.volume,
            pitch_hz: preset.pitch_hz,
            beep_duration: preset.beep_duration,
            is_running: false, // Should not start running on load
            visual_enabled: preset.visual_enabled,
            last_beat: None,
            tuning: preset.tuning,
        }
    }
}

impl Metronome {
    pub fn to_preset(&self) -> MetronomePreset {
        MetronomePreset {
            bpm: self.bpm,
            volume: self.volume,
            pitch_hz: self.pitch_hz,
            beep_duration: self.beep_duration,
            visual_enabled: self.visual_enabled,
            tuning: self.tuning.clone(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets::preset::Tuning;

    #[test]
    fn test_from_preset() {
        let preset = MetronomePreset {
            bpm: 150.0,
            volume: 0.6,
            pitch_hz: 440.0,
            beep_duration: 0.01,
            visual_enabled: false,
            tuning: Tuning {
                reference_pitch: 440.0,
                octave: 4,
                note_index: 9,
            },
        };

        let metronome = Metronome::from(preset.clone());
        assert_eq!(metronome.bpm, 150.0);
        assert_eq!(metronome.volume, 0.6);
        assert_eq!(metronome.pitch_hz, 440.0);
        assert_eq!(metronome.beep_duration, 0.01);
        assert_eq!(metronome.visual_enabled, false);
        assert_eq!(metronome.is_running, false); // Important: should be false
        assert_eq!(metronome.tuning, preset.tuning);
    }

    #[test]
    fn test_to_preset() {
        let metronome = Metronome {
            bpm: 120.0,
            volume: 0.5,
            pitch_hz: 880.0,
            beep_duration: 0.02,
            is_running: true,
            visual_enabled: true,
            last_beat: None,
            tuning: Tuning {
                reference_pitch: 440.0,
                octave: 5,
                note_index: 9,
            },
        };

        let preset = metronome.to_preset();
        assert_eq!(preset.bpm, 120.0);
        assert_eq!(preset.volume, 0.5);
        assert_eq!(preset.pitch_hz, 880.0);
        assert_eq!(preset.beep_duration, 0.02);
        assert_eq!(preset.visual_enabled, true);
        assert_eq!(preset.tuning, metronome.tuning);
    }
}
