// Handles the audio playback thread and sound generation.
use crate::domain::metronome::Metronome;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(not(feature = "audio"))]
use crate::audio::mock as audio_crate;
#[cfg(feature = "audio")]
use rodio as audio_crate;

use audio_crate::{Sink, Source, mixer::Mixer, source::SineWave};

pub fn start_metronome_thread(state: Arc<Mutex<Metronome>>, mixer: Mixer, stop: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut next_tick = Instant::now();

        loop {
            // Check the per-session stop token before acquiring the lock so that
            // a new session starting while this thread sleeps causes an immediate exit.
            if stop.load(Ordering::Relaxed) {
                break;
            }

            let (bpm, volume, pitch, beep_duration, is_running) = {
                let mut state_guard = state.lock().unwrap();
                if !state_guard.is_running {
                    break;
                }
                if Instant::now() >= next_tick {
                    state_guard.last_beat = Some(Instant::now());
                }
                (
                    state_guard.bpm,
                    state_guard.volume,
                    state_guard.pitch_hz,
                    state_guard.beep_duration,
                    state_guard.is_running,
                )
            };

            if !is_running {
                break;
            }

            if Instant::now() >= next_tick {
                let sink = Sink::connect_new(&mixer);

                let interval = 60.0 / bpm;
                let max_dur = interval * 0.95;
                let target_dur = beep_duration.min(max_dur);

                let cycles = (target_dur * pitch).round().max(1.0);
                let duration = Duration::from_secs_f32(cycles / pitch);

                let source = SineWave::new(pitch).take_duration(duration).amplify(volume);

                sink.append(source);
                sink.detach();

                next_tick += Duration::from_secs_f32(interval);

                if next_tick < Instant::now() {
                    next_tick = Instant::now() + Duration::from_secs_f32(interval);
                }
            }

            let now = Instant::now();
            if let Some(sleep_duration) = next_tick.checked_duration_since(now) {
                thread::sleep(sleep_duration.min(Duration::from_millis(10)));
            }
        }
    });
}

#[cfg(all(test, not(feature = "audio")))]
mod tests {
    use super::*;
    use crate::domain::metronome::Metronome;
    use crate::presets::preset::MetronomePreset;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_start_stop_metronome_thread() {
        let state = Arc::new(Mutex::new(Metronome::from(MetronomePreset::default())));
        state.lock().unwrap().is_running = true;

        let mixer = Mixer;
        let stop = Arc::new(AtomicBool::new(false));
        start_metronome_thread(state.clone(), mixer, stop);

        std::thread::sleep(Duration::from_millis(50));
        state.lock().unwrap().is_running = false;
        // The thread should exit now
        std::thread::sleep(Duration::from_millis(50));
    }
}
