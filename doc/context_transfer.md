# PROJECT CONTEXT INJECTION

# PROJECT TYPE
Desktop GUI application — Rust, edition 2024.

# PROJECT VISION
Savage Metronome: a native desktop metronome with tunable pitch, tap tempo, visual beat indicator, volume control, beep duration control, and preset save/load. Targets Windows and macOS.

# ARCHITECTURE
Layered — strict dependency direction: UI → Audio/Presets → Domain. Domain has zero external deps.

- `src/domain/` — pure business logic (Metronome state, tap tempo, time signature placeholder)
- `src/audio/` — beat scheduling thread, sine wave generation via rodio; feature-gated (`audio`)
- `src/presets/` — JSON serialization of MetronomePreset; persisted to platform config dirs via `directories`
- `src/ui/` — egui/eframe immediate-mode GUI; MetronomeApp holds Arc<Mutex<Metronome>>
- `src/config/` — placeholder for app-wide settings

Shared state between UI and audio thread: `Arc<Mutex<Metronome>>`.

# INVARIANTS
- Domain layer has zero UI/audio dependencies.
- Audio scheduling runs on its own thread, independent of the UI thread.
- Preset JSON format must stay backward-compatible.
- Timing accuracy is prioritized over UI responsiveness.

# TECHNICAL DECISIONS
- **rodio 0.21.1 API**: `OutputStreamBuilder::open_default_stream()` returns `OutputStream`; `Mixer` (from `stream.mixer().clone()`) is the shared handle passed to audio thread. `Sink::connect_new(&mixer)` creates sinks. `take_duration` / `amplify` are provided methods on the `Source` trait (imported as `use rodio::Source`).
- **Mock audio API** (`src/audio/mock.rs`): mirrors the rodio surface used by `engine.rs` and `app.rs`. The top-level `Source` trait provides `take_duration` and `amplify` as provided methods, matching rodio's design. Import as `use audio_crate::Source` in engine.rs.
- **egui 0.33.3**: `DragValue::range()` (not `clamp_range`), `ComboBox::from_id_salt()` (not `from_id_source`), `rect_stroke` requires 4th `StrokeKind` argument, `run_native` closure returns `Result`.
- **Equal temperament**: pitch = `440 * 2^((note_index + octave*12 - 69) / 12)` Hz.
- **Windows icon embedding**: `embed-resource = "2"` build dependency + `build.rs` calls `embed_resource::compile("windows.rc", embed_resource::NONE)`. Resource file: `windows.rc` → `1 ICON "assets/savage_metronome.ico"`.
- **macOS bundling**: `cargo-bundle` subcommand reads `[package.metadata.bundle]` in Cargo.toml. Icon: `assets/savage_metronome.icns`. Run with `cargo bundle --release`.

# MODULE MAP
- `src/main.rs` — entry point, window size, `eframe::run_native`
- `src/ui/app.rs` — MetronomeApp struct + eframe::App impl (all UI rendering)
- `src/audio/engine.rs` — `start_metronome_thread(state, mixer)` — beat loop, sine wave playback
- `src/audio/mock.rs` — headless mock of the rodio API (`Mixer`, `Sink`, `Source`, `SineWave`, `OutputStream`); used when the `audio` feature is disabled
- `src/domain/metronome.rs` — Metronome struct (bpm, volume, pitch_hz, beep_duration, visual_enabled, is_running, last_beat, tuning)
- `src/domain/tempo.rs` — `calculate_tap_tempo(times)` — averages up to 8 tap intervals
- `src/presets/preset.rs` — MetronomePreset, Tuning, NOTE_NAMES, `calculate_pitch`
- `src/presets/storage.rs` — `save_preset` / `load_preset` using serde_json + directories
- `build.rs` — embeds Windows icon resource via embed-resource
- `windows.rc` — Windows resource script pointing to `assets/savage_metronome.ico`

# CURRENT FOCUS
- Build is clean. All tests pass (7 with audio feature, 8 without).
- Last major work (cfd8658): added unit tests for preset storage and domain logic; added mock audio backend for headless/no-audio testing.
- The engine test (`test_start_stop_metronome_thread`) is gated `#[cfg(all(test, not(feature = "audio")))]` so it only runs with the mock backend.

# OPERATIONAL CONSTRAINTS
- `cargo build` / `cargo build --release` — standard build
- `cargo bundle --release` — macOS .app bundle (requires `cargo install cargo-bundle`)
- `cargo test` — all unit + integration tests
- Audio is an optional feature: `cargo build --no-default-features` disables rodio
