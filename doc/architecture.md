# Application Type
Single Rust binary with internal modularization.

# Layered Concept
1. Domain Layer
- Tempo logic
- Time signature logic
- Tick scheduling math
- Metronome state machine
- No side effects
2. Audio Layer
- Audio backend abstraction
- WAV loading
- Playback engine
- Timer → sound triggering
3. Preset Layer
- Serialization/deserialization
- Preset validation
- File I/O abstraction
4. UI Layer
- Rendering
- Input handling
- State management
- UI event loop
5. Infrastructure
- File system access
- Config loading
- Logging
- Dependency Direction

# Dependency Direction
- UI → Domain
- UI → Audio
- Presets → Domain
- Audio → Domain
- Domain depends on nothing.
