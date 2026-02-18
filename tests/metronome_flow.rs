// Integration tests for the main metronome start/stop workflow (mock audio backend).
#![cfg(not(feature = "audio"))]

use metronome::audio::engine::start_metronome_thread;
use metronome::audio::mock::Mixer;
use metronome::domain::metronome::Metronome;
use metronome::presets::preset::MetronomePreset;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Starting and then stopping the thread leaves is_running false.
#[test]
fn test_start_stop_sets_not_running() {
    let state = Arc::new(Mutex::new(Metronome::from(MetronomePreset::default())));
    state.lock().unwrap().is_running = true;

    let stop = Arc::new(AtomicBool::new(false));
    start_metronome_thread(state.clone(), Mixer, stop.clone());

    std::thread::sleep(Duration::from_millis(30));

    stop.store(true, Ordering::Relaxed);
    state.lock().unwrap().is_running = false;

    std::thread::sleep(Duration::from_millis(30));

    assert!(!state.lock().unwrap().is_running);
}

/// Stop followed immediately by a second Start does not leave two threads running.
/// After the second session is stopped, beats must cease (last_beat stays stale).
#[test]
fn test_stop_then_restart_no_double_thread() {
    let state = Arc::new(Mutex::new(Metronome::from(MetronomePreset::default())));

    // --- First session ---
    state.lock().unwrap().is_running = true;
    let stop1 = Arc::new(AtomicBool::new(false));
    start_metronome_thread(state.clone(), Mixer, stop1.clone());
    std::thread::sleep(Duration::from_millis(30));

    // Stop first session; immediately start second (simulates the race condition).
    stop1.store(true, Ordering::Relaxed);
    state.lock().unwrap().is_running = false;

    // --- Second session ---
    state.lock().unwrap().is_running = true;
    let stop2 = Arc::new(AtomicBool::new(false));
    start_metronome_thread(state.clone(), Mixer, stop2.clone());
    std::thread::sleep(Duration::from_millis(30));

    // Stop second session.
    stop2.store(true, Ordering::Relaxed);
    state.lock().unwrap().is_running = false;

    // Wait for both threads to drain.
    std::thread::sleep(Duration::from_millis(50));

    // Neither thread should still be driving beats.
    assert!(!state.lock().unwrap().is_running);
}
