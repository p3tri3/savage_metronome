# Guidelines
- Each file should have 1–2 line description for a model to orient itself.

# Module map (as folders and files)

src/
  main.rs           # Binary entry point. Initializes the application.
  lib.rs            # Library root, declares all modules.
  domain/
    mod.rs          # Declares domain sub-modules.
    metronome.rs    # Core runtime state and behavior of the metronome.
    tempo.rs        # Logic for tempo calculations (e.g., tap tempo).
    time_signature.rs # Placeholder for time signature logic.
  audio/
    mod.rs          # Declares audio sub-modules.
    engine.rs       # Audio thread for sound generation and timing.
    wav_loader.rs   # Placeholder for loading custom WAV sounds.
  presets/
    mod.rs          # Declares preset sub-modules.
    preset.rs       # Data structure for a single, serializable preset.
    storage.rs      # Handles saving/loading presets to/from the filesystem.
  ui/
    mod.rs          # Declares UI sub-modules.
    app.rs          # Main application struct and eframe::App implementation.
    events.rs       # Placeholder for UI event definitions.
  config/
    mod.rs          # Declares config sub-modules.
    settings.rs     # Placeholder for application-wide settings.

tests/
  preset_integration.rs # Tests saving and loading of presets.
  metronome_flow.rs     # Placeholder for end-to-end metronome tests.
