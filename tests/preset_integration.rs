// Integration test for saving and loading presets to a file path.
use metronome::presets::{
    preset::MetronomePreset,
    storage::{load_preset_from_path, save_preset_to_path},
};
use tempfile::tempdir;

#[test]
fn test_save_and_load_preset_to_path() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_preset.json");

    let preset_to_save = MetronomePreset {
        bpm: 123.0,
        volume: 0.75,
        ..Default::default()
    };

    // Save the preset
    let save_result = save_preset_to_path(&preset_to_save, &file_path);
    assert!(save_result.is_ok());

    // Load the preset
    let load_result = load_preset_from_path(&file_path);
    assert!(load_result.is_ok());

    let loaded_preset = load_result.unwrap();

    assert_eq!(preset_to_save, loaded_preset);
}