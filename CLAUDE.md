# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Savage Metronome — a desktop metronome app built with Rust (edition 2024), using egui/eframe for UI and rodio for audio.

## Build & Test Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo run --release            # Run the app
cargo test                     # Run all tests (unit + integration)
cargo test --test preset_integration  # Run a single integration test
cargo test tempo::tests        # Run unit tests in a specific module
cargo build --no-default-features     # Build without audio (rodio disabled)
```

## Architecture

Layered architecture with strict dependency direction: **UI → Audio/Presets → Domain**. Domain depends on nothing.

### Layers

- **Domain** (`src/domain/`) — Pure business logic: metronome state (`metronome.rs`), tap tempo calculation (`tempo.rs`), time signatures (placeholder). No side effects, no UI/audio dependencies.
- **Audio** (`src/audio/`) — Spawns a separate thread for beat scheduling and sine wave generation via rodio. Reads shared state through `Arc<Mutex<Metronome>>`. Feature-gated behind `audio` cargo feature.
- **Presets** (`src/presets/`) — JSON serialization/deserialization of `MetronomePreset` structs. Stores to platform-specific config dirs via the `directories` crate.
- **UI** (`src/ui/`) — egui immediate-mode GUI in `app.rs`. `MetronomeApp` implements `eframe::App`, holds `Arc<Mutex<Metronome>>` for shared state with the audio thread.
- **Config** (`src/config/`) — Placeholder for app-wide settings.

### Key Patterns

- **Shared state**: `Arc<Mutex<Metronome>>` shared between UI and audio threads.
- **Equal temperament tuning**: `440Hz * 2^((n-69)/12)` for pitch calculation from note index.
- **Tap tempo**: Stores up to 8 tap timestamps, averages intervals, converts to BPM.
- **Audio feature gate**: `rodio` is optional via `[features] audio = ["dep:rodio"]`.

### Invariants

- Domain layer must have zero dependencies on UI or audio.
- Audio scheduling must not depend on the UI thread.
- Preset JSON format must maintain backward compatibility.
- Timing accuracy is prioritized over UI responsiveness.

## Testing

- **Unit tests**: Inside source files via `#[cfg(test)]` modules (e.g., `tempo.rs`, `preset.rs`).
- **Integration tests**: In `tests/` directory. `preset_integration.rs` tests save/load cycles using `tempfile`.
- Domain logic is tested without UI or audio backend. Tests must be deterministic.
