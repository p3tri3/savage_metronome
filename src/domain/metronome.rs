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