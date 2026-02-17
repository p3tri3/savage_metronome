// Handles the audio playback thread and sound generation.
use crate::domain::metronome::Metronome;
use rodio::{mixer::Mixer, Sink, Source};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_metronome_thread(state: Arc<Mutex<Metronome>>, mixer: Mixer) {
    thread::spawn(move || {
        let mut next_tick = Instant::now();

        loop {
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

                let source = rodio::source::SineWave::new(pitch)
                    .take_duration(duration)
                    .amplify(volume);
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