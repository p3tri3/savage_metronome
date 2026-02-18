# AGENTS.md

Repository guidance for **AI coding agents**.

## Project

**Savage Metronome** — native desktop metronome app in **Rust (edition 2024)** with:
- **UI:** egui/eframe
- **Audio:** rodio (optional Cargo feature `audio`)
- **Presets:** JSON persisted to platform config dirs (`directories` crate)

Targets: Windows + macOS.

## Quick commands

```bash
cargo build
cargo build --release
cargo run --release

cargo test
cargo build --no-default-features   # audio disabled (mock backend)
```

macOS app bundle (requires `cargo install cargo-bundle`):
```bash
cargo bundle --release
```

## Architecture

Layered with strict dependency direction:

**UI → Audio/Presets → Domain**

- `src/domain/` — **pure business logic**, no side effects, **zero external deps**
- `src/audio/` — beat scheduling thread + sine generation (rodio), behind feature `audio`
- `src/presets/` — `MetronomePreset` JSON (serde) + storage via `directories`
- `src/ui/` — egui/eframe app
- `src/config/` — placeholder

Shared state between UI and audio thread: `Arc<Mutex<Metronome>>`.

## Invariants (do not violate)

- **Domain stays pure:** no UI/audio/persistence dependencies.
- **Audio thread independence:** beat scheduling must not depend on UI thread responsiveness.
- **Preset backward compatibility:** existing preset JSON must continue to load.
- **Preset sanitization on load:** `MetronomePreset::sanitize()` must be called on every deserialized preset (already done in `load_preset_from_path`). Do not bypass it.
- **Timing accuracy > UI responsiveness** when tradeoffs are required.

## Editing guardrails

- Prefer **small, targeted changes**; avoid broad refactors unless requested.
- If changing preset schema/serialization: implement **backward-compatible parsing** and add tests.
- If touching audio/threading: keep synchronization minimal; avoid UI-thread blocking.
- **Stop-token pattern:** always create a fresh `Arc<AtomicBool>` stop token when starting the audio thread and store it in `MetronomeApp::thread_stop`. Signal `true` on Stop before clearing `is_running` so old threads exit without racing against a new session.
- Keep tests deterministic; do not introduce timing-flaky tests.

## Feature gating

- Audio is optional. Building with `--no-default-features` disables rodio and uses the mock backend.
- Ensure changes compile and tests pass in both modes when relevant:
  - `cargo test`
  - `cargo test --no-default-features`

## Version-specific API notes (avoid “hallucinated” calls)

### rodio (as used here)
- `OutputStreamBuilder::open_default_stream()` returns `OutputStream`
- Obtain mixer from `stream.mixer().clone()` and pass to the audio thread
- Create sinks via `Sink::connect_new(&mixer)`
- `take_duration` / `amplify` are provided methods on the `Source` trait (import `use rodio::Source`)

Mock backend (`src/audio/mock.rs`) mirrors the rodio surface used by `engine.rs` and `app.rs`.

### egui/eframe (as used here)
- `DragValue::range()` (not `clamp_range`)
- `ComboBox::from_id_salt()` (not `from_id_source`)
- `rect_stroke` requires a 4th `StrokeKind` argument
- `run_native` closure returns a `Result`

## Module map

- `src/main.rs` — entry point, window config, `eframe::run_native`
- `src/ui/app.rs` — `MetronomeApp` + all UI rendering
- `src/audio/engine.rs` — `start_metronome_thread(state, mixer, stop)` beat loop + playback; `stop: Arc<AtomicBool>` is a per-session stop token (set to `true` to terminate the thread)
- `src/audio/mock.rs` — mock `Mixer/Sink/Source/SineWave/OutputStream` when audio feature disabled
- `src/domain/metronome.rs` — `Metronome` state
- `src/domain/tempo.rs` — `calculate_tap_tempo(times)` (avg up to 7 intervals; ring buffer holds 8 timestamps)
- `src/presets/preset.rs` — `MetronomePreset`, tuning, pitch calculation
- `src/presets/storage.rs` — `save_preset` / `load_preset`
- `build.rs` — Windows icon embedding via `embed-resource`
- `windows.rc` — icon resource script (`assets/savage_metronome.ico`)

## Platform packaging notes

- **Windows icon embedding:** `embed-resource = "2"` + `build.rs` compiles `windows.rc`
- **macOS bundling:** `cargo-bundle` reads `[package.metadata.bundle]` in `Cargo.toml`
  - icon: `assets/savage_metronome.icns`

## Validation checklist (before concluding work)

- `cargo fmt` (if project uses rustfmt)
- `cargo test` (default features)
- `cargo test --no-default-features` (mock backend)
- If you changed presets: run/extend preset integration tests in `tests/`
- If you changed audio/threading: ensure start/stop behavior remains correct in mock mode tests
