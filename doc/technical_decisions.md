# Language
- Rust stable edition (1.93.1)

# Project Structure
- Binary + internal modules
- Possible future migration to workspace

# Testing Strategy
- Unit tests inside modules
- Integration tests in tests/
- Domain logic must remain testable without UI or audio backend

# Audio Strategy
- Audio backend abstracted behind trait
- WAV loading separated from playback engine

# Serialization
- Use serde for preset serialization
- JSON format for presets (human-readable)

# Timing Strategy
- Timing accuracy prioritized over UI responsiveness
- Audio scheduling must not depend on UI thread
