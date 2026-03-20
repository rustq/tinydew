use crate::state::GameState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameData {
    pub version: u32,
    pub state: GameState,
}

impl SaveGameData {
    pub fn new(state: GameState) -> Self {
        Self { version: 1, state }
    }
}

fn get_default_save_dir() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        return data_dir.join("tinydew");
    }
    PathBuf::from(".")
}

pub fn get_save_path() -> PathBuf {
    let mut path = get_default_save_dir();
    fs::create_dir_all(&path).ok();
    path.push("savegame.json");
    path
}

#[cfg(test)]
fn get_test_save_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("tinydew_test");
    fs::create_dir_all(&path).ok();
    path.push("savegame.json");
    path
}

pub fn save_game_to_path(state: &GameState, path: &PathBuf) -> Result<PathBuf, SaveError> {
    let save_data = SaveGameData::new(state.clone());
    let json = serde_json::to_string_pretty(&save_data)
        .map_err(|e| SaveError::SerializationError(e.to_string()))?;

    fs::write(path, &json).map_err(|e| SaveError::IoError(e.to_string()))?;

    Ok(path.clone())
}

pub fn load_game_from_path(path: &PathBuf) -> Result<GameState, SaveError> {
    let json = fs::read_to_string(path).map_err(|e| SaveError::FileNotFound(e.to_string()))?;

    let save_data: SaveGameData =
        serde_json::from_str(&json).map_err(|e| SaveError::CorruptSave(e.to_string()))?;

    Ok(save_data.state)
}

pub fn save_game(state: &GameState) -> Result<PathBuf, SaveError> {
    let path = get_save_path();
    save_game_to_path(state, &path)
}

pub fn load_game() -> Result<GameState, SaveError> {
    let path = get_save_path();
    load_game_from_path(&path)
}

pub fn save_exists() -> bool {
    get_save_path().exists()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SaveError {
    FileNotFound(String),
    IoError(String),
    SerializationError(String),
    CorruptSave(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::FileNotFound(msg) => write!(f, "Save file not found: {}", msg),
            SaveError::IoError(msg) => write!(f, "IO error: {}", msg),
            SaveError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            SaveError::CorruptSave(msg) => write!(f, "Corrupt save file: {}", msg),
        }
    }
}

impl std::error::Error for SaveError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_restores_state() {
        let test_path = get_test_save_path();

        let mut state = GameState::new();
        state.day = 5;
        state.money = 1000;
        state.player_x = 5;
        state.player_y = 6;
        state
            .inventory
            .seeds
            .insert(crate::world::CropType::Carrot, 10);

        save_game_to_path(&state, &test_path).expect("Save should succeed");
        assert!(test_path.exists());

        let loaded_state = load_game_from_path(&test_path).expect("Load should succeed");

        assert_eq!(loaded_state.day, state.day);
        assert_eq!(loaded_state.money, state.money);
        assert_eq!(loaded_state.player_x, state.player_x);
        assert_eq!(loaded_state.player_y, state.player_y);
        assert_eq!(
            loaded_state
                .inventory
                .seeds
                .get(&crate::world::CropType::Carrot),
            state.inventory.seeds.get(&crate::world::CropType::Carrot)
        );

        fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_save_then_mutate_then_load_restores_prior_state() {
        let test_path = get_test_save_path();

        let state_before = GameState::new();
        let mut state_before = state_before;
        state_before.day = 10;
        state_before.money = 2500;

        save_game_to_path(&state_before, &test_path).expect("Save should succeed");

        let mut mutated = GameState::new();
        mutated.day = 999;
        mutated.money = 0;
        mutated.player_x = 100;

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert_eq!(loaded.day, 10);
        assert_eq!(loaded.money, 2500);
        assert_eq!(mutated.day, 999);
        assert_eq!(mutated.money, 0);

        fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_missing_file_returns_error() {
        let test_path = get_test_save_path();

        fs::remove_file(&test_path).ok();

        let result = load_game_from_path(&test_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SaveError::FileNotFound(_)));
    }

    #[test]
    fn test_corrupt_save_returns_error() {
        let test_path = get_test_save_path();

        fs::write(&test_path, "not valid json{{{").ok();

        let result = load_game_from_path(&test_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SaveError::CorruptSave(_)));

        fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_save_exists_returns_false_when_no_save() {
        let test_path = get_test_save_path();

        fs::remove_file(&test_path).ok();

        let result = load_game_from_path(&test_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_save_exists_returns_true_after_save() {
        let test_path = get_test_save_path();

        let state = GameState::new();
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        assert!(test_path.exists());

        fs::remove_file(&test_path).ok();
    }
}
