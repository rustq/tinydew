use crate::savegame::{self, SaveError};
use crate::state::GameState;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

pub const SINGLETON_SESSION_ID: &str = "singleton";

pub struct GameStateManager {
    pub state: GameState,
    pub last_save_time: Option<chrono::DateTime<chrono::Utc>>,
    pub dirty: bool,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            last_save_time: None,
            dirty: false,
        }
    }

    pub fn load_or_new() -> Self {
        match savegame::load_game() {
            Ok(state) => {
                info!("Loaded save game from {:?}", savegame::get_save_path());
                Self {
                    state,
                    last_save_time: Some(chrono::Utc::now()),
                    dirty: false,
                }
            }
            Err(e) => {
                match &e {
                    SaveError::FileNotFound(_) => {
                        info!("No save file found, starting new game");
                    }
                    SaveError::CorruptSave(msg) => {
                        warn!("Corrupt save file detected: {}, starting fresh", msg);
                    }
                    _ => {
                        warn!("Failed to load save: {}, starting new game", e);
                    }
                }
                Self::new()
            }
        }
    }

    pub fn save(&mut self) -> Result<Option<String>, SaveError> {
        let path = savegame::get_save_path();
        savegame::save_game_to_path(&self.state, &path)?;
        self.last_save_time = Some(chrono::Utc::now());
        self.dirty = false;
        info!("Autosaved game to {:?}", path);
        Ok(Some(path.to_string_lossy().to_string()))
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn to_snapshot(&self) -> serde_json::Value {
        let mut snapshot = serde_json::json!({
            "day": self.state.day,
            "time": self.state.format_time(),
            "weather": format!("{:?}", self.state.weather),
            "weather_day": self.state.weather_day,
            "is_paused": self.state.is_paused,
            "location": format!("{:?}", self.state.location),
            "money": self.state.money,
            "message": self.state.message,
            "inventory": {
                "seeds": self.state.inventory.seeds,
                "produce": self.state.inventory.produce,
                "forage": self.state.inventory.forage,
            },
            "player": {
                "x": self.state.player_x,
                "y": self.state.player_y,
                "location": format!("{:?}", self.state.player_location),
            },
            "status": "ok"
        });

        if self.state.guest_enabled {
            snapshot["guest"] = serde_json::json!({
                "enabled": self.state.guest_enabled,
                "x": self.state.guest_x,
                "y": self.state.guest_y,
                "location": format!("{:?}", self.state.guest_location),
                "active": self.state.active_control == crate::state::ControlTarget::Guest,
            });
        }

        snapshot
    }

    pub fn to_map_snapshot(&self, include_entities: bool) -> serde_json::Value {
        let map = self.state.get_current_map_ref();
        let (width, height) = self.state.get_map_size();

        let rows: Vec<Vec<String>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let tile = &map[y][x];
                        if include_entities {
                            Self::tile_to_detailed_string(tile)
                        } else {
                            Self::tile_to_code(tile)
                        }
                    })
                    .collect()
            })
            .collect();

        serde_json::json!({
            "location": format!("{:?}", self.state.location),
            "width": width,
            "height": height,
            "player_x": self.state.player_x,
            "player_y": self.state.player_y,
            "tiles": rows
        })
    }

    fn tile_to_code(tile: &crate::world::TileType) -> String {
        use crate::world::TileType;
        match tile {
            TileType::Boundary => "X".to_string(),
            TileType::Grass => ".".to_string(),
            TileType::Soil => "#".to_string(),
            TileType::Crop(_, _) => "C".to_string(),
            TileType::House => "H".to_string(),
            TileType::PathEast => "P".to_string(),
            TileType::PathFarm => "P".to_string(),
            TileType::PathSquare => "P".to_string(),
            TileType::PathSouthRiver => "P".to_string(),
            TileType::PathSouthRiverGate => "P".to_string(),
            TileType::Player => "@".to_string(),
            TileType::Mushroom => "M".to_string(),
            TileType::Fountain => "F".to_string(),
            TileType::Slide => "S".to_string(),
            TileType::River => "R".to_string(),
            TileType::RiverBubble => "B".to_string(),
            TileType::Wonder => "W".to_string(),
        }
    }

    fn tile_to_detailed_string(tile: &crate::world::TileType) -> String {
        use crate::world::{CropType, TileType};
        match tile {
            TileType::Boundary => "Boundary".to_string(),
            TileType::Grass => "Grass".to_string(),
            TileType::Soil => "Soil".to_string(),
            TileType::Crop(crop, state) => {
                let growth = state.days_grown;
                let mature = if state.is_mature(*crop) {
                    "mature"
                } else {
                    "growing"
                };
                format!("Crop({:?},{}d,{})", crop, growth, mature)
            }
            TileType::House => "House".to_string(),
            TileType::PathEast => "PathEast".to_string(),
            TileType::PathFarm => "PathFarm".to_string(),
            TileType::PathSquare => "PathSquare".to_string(),
            TileType::PathSouthRiver => "PathSouthRiver".to_string(),
            TileType::PathSouthRiverGate => "PathSouthRiverGate".to_string(),
            TileType::Player => "Player".to_string(),
            TileType::Mushroom => "Mushroom".to_string(),
            TileType::Fountain => "Fountain".to_string(),
            TileType::Slide => "Slide".to_string(),
            TileType::River => "River".to_string(),
            TileType::RiverBubble => "RiverBubble".to_string(),
            TileType::Wonder => "Wonder".to_string(),
        }
    }

    pub fn to_inventory_snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "seeds": self.state.inventory.seeds,
            "produce": self.state.inventory.produce,
            "forage": self.state.inventory.forage,
            "fish": self.state.inventory.fish,
            "money": self.state.money,
            "selected_seed": format!("{:?}", self.state.selected_seed),
        })
    }

    pub fn to_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "day": self.state.day,
            "time": self.state.format_time(),
            "location": format!("{:?}", self.state.location),
            "money": self.state.money,
            "inventory": {
                "seeds": self.state.inventory.seeds,
                "produce": self.state.inventory.produce,
                "forage": self.state.inventory.forage,
            },
            "status": "ok",
            "season": self.state.season,
            "weather": format!("{:?}", self.state.weather),
            "heartbeat": {
                "is_paused": self.state.is_paused,
            }
        })
    }

    pub fn get_day_and_time(&self) -> (u32, u8, u8) {
        self.state.get_day_and_time()
    }
}

impl Default for GameStateManager {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedGameState = Arc<Mutex<GameStateManager>>;

pub fn create_shared_state() -> SharedGameState {
    Arc::new(Mutex::new(GameStateManager::load_or_new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::savegame;

    #[test]
    fn test_singleton_state_new() {
        let manager = GameStateManager::new();
        assert_eq!(manager.state.day, 1);
        assert_eq!(manager.state.money, 500);
        assert!(!manager.dirty);
    }

    #[test]
    fn test_to_snapshot() {
        let manager = GameStateManager::new();
        let snapshot = manager.to_snapshot();
        assert_eq!(snapshot.get("day").unwrap().as_u64().unwrap(), 1);
        assert_eq!(snapshot.get("status").unwrap(), "ok");
    }

    #[test]
    fn test_to_snapshot_includes_guest_when_enabled() {
        let mut manager = GameStateManager::new();
        manager.state.guest_enabled = true;
        manager.state.guest_x = 4;
        manager.state.guest_y = 2;
        manager.state.guest_location = crate::state::Location::Square;
        manager.state.active_control = crate::state::ControlTarget::Guest;

        let snapshot = manager.to_snapshot();
        let guest = snapshot
            .get("guest")
            .expect("guest block should exist when guest is enabled");

        assert_eq!(guest.get("x").and_then(|v| v.as_u64()), Some(4));
        assert_eq!(guest.get("y").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(
            guest.get("location").and_then(|v| v.as_str()),
            Some("Square")
        );
        assert_eq!(guest.get("active").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn test_mark_dirty() {
        let mut manager = GameStateManager::new();
        assert!(!manager.dirty);
        manager.mark_dirty();
        assert!(manager.dirty);
    }

    #[test]
    fn test_autosave_persists_state() {
        let test_path: std::path::PathBuf =
            std::env::temp_dir().join("shelldew_autosave_test.json");

        let mut state = GameState::new();
        state.day = 5;
        state.money = 1000;
        state
            .inventory
            .seeds
            .insert(crate::world::CropType::Carrot, 10);

        savegame::save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = savegame::load_game_from_path(&test_path).expect("Load should succeed");
        assert_eq!(loaded.day, 5);
        assert_eq!(loaded.money, 1000);

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_corrupt_save_fallback() {
        let test_path: std::path::PathBuf = std::env::temp_dir().join("shelldew_corrupt_test.json");

        std::fs::write(&test_path, "not valid json{{{").ok();

        let loaded = savegame::load_game_from_path(&test_path);
        assert!(loaded.is_err());

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let test_path: std::path::PathBuf =
            std::env::temp_dir().join("shelldew_roundtrip_test.json");

        let mut state = GameState::new();
        state.day = 10;
        state.money = 5000;
        state
            .inventory
            .produce
            .insert(crate::world::CropType::Strawberry, 20);

        savegame::save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = savegame::load_game_from_path(&test_path).expect("Load should succeed");
        assert_eq!(loaded.day, 10);
        assert_eq!(loaded.money, 5000);
        assert_eq!(
            loaded
                .inventory
                .produce
                .get(&crate::world::CropType::Strawberry),
            Some(&20)
        );

        std::fs::remove_file(&test_path).ok();
    }
}
