// Manages saving and loading of presets to the filesystem.
use super::preset::MetronomePreset;
use directories::ProjectDirs;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const PRESET_FILE_NAME: &str = "metronome_preset.json";

fn get_preset_path() -> io::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "savagemetronome", "Savage Metronome") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        Ok(config_dir.join(PRESET_FILE_NAME))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find a valid home directory.",
        ))
    }
}

pub fn save_preset(preset: &MetronomePreset) -> Result<(), io::Error> {
    let path = get_preset_path()?;
    save_preset_to_path(preset, &path)
}

pub fn load_preset() -> Result<MetronomePreset, io::Error> {
    let path = get_preset_path()?;
    load_preset_from_path(&path)
}

pub fn save_preset_to_path(preset: &MetronomePreset, path: &Path) -> Result<(), io::Error> {
    let json_string =
        serde_json::to_string_pretty(preset).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, json_string)
}

pub fn load_preset_from_path(path: &Path) -> Result<MetronomePreset, io::Error> {
    let json_string = fs::read_to_string(path)?;
    serde_json::from_str(&json_string).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_non_existent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_existent.json");
        let result = load_preset_from_path(&file_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_load_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.json");
        fs::write(&file_path, "{ \"invalid\": json }").unwrap();
        let result = load_preset_from_path(&file_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_save_and_load_preset_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("preset.json");
        let preset = MetronomePreset {
            bpm: 144.0,
            volume: 0.8,
            ..Default::default()
        };

        save_preset_to_path(&preset, &file_path).unwrap();
        let loaded = load_preset_from_path(&file_path).unwrap();
        assert_eq!(preset, loaded);
    }
}
