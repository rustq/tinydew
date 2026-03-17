use crate::state::GameState;
use crate::world::TileType;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use super::errors::McpError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub day: u32,
    pub time: String,
    pub action: String,
    pub result: String,
}

pub struct Session {
    pub id: String,
    pub game_state: GameState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub closed: bool,
    pub logs: Vec<LogEntry>,
}

impl Session {
    pub fn new(seed: Option<u64>, mode: Option<String>) -> Self {
        let mut game_state = GameState::new();

        if let Some(_seed) = seed {
            // Future: deterministic seeding support
        }

        if let Some(m) = mode {
            game_state.message = format!("Started in {} mode", m);
        }

        let initial_log = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            day: game_state.day,
            time: game_state.format_time(),
            action: "session_start".to_string(),
            result: "Session created".to_string(),
        };

        Self {
            id: Uuid::new_v4().to_string(),
            game_state,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            closed: false,
            logs: vec![initial_log],
        }
    }

    pub fn to_snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "day": self.game_state.day,
            "time": self.game_state.format_time(),
            "location": format!("{:?}", self.game_state.location),
            "money": self.game_state.money,
            "inventory": {
                "seeds": self.game_state.inventory.seeds,
                "produce": self.game_state.inventory.produce,
                "forage": self.game_state.inventory.forage,
            },
            "player": {
                "x": self.game_state.player_x,
                "y": self.game_state.player_y,
            },
            "status": if self.closed { "closed" } else { "ok" }
        })
    }

    /// Serialize map to a stable format.
    /// Format: 2D array of tile strings.
    /// Each tile is represented by a short code:
    /// - "X": Boundary
    /// - ".": Grass
    /// - "#": Soil
    /// - "H": House
    /// - "P": PathEast/PathFarm
    /// - "M": Mushroom
    /// - "C": Crop (generic)
    /// When include_entities is true, crops show their type and growth stage.
    pub fn to_map_snapshot(&self, include_entities: bool) -> serde_json::Value {
        let map = self.game_state.get_current_map_ref();
        let (width, height) = self.game_state.get_map_size();

        let rows: Vec<Vec<String>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let tile = &map[y][x];
                        if include_entities {
                            self.tile_to_detailed_string(tile)
                        } else {
                            self.tile_to_code(tile)
                        }
                    })
                    .collect()
            })
            .collect();

        serde_json::json!({
            "location": format!("{:?}", self.game_state.location),
            "width": width,
            "height": height,
            "player_x": self.game_state.player_x,
            "player_y": self.game_state.player_y,
            "tiles": rows
        })
    }

    fn tile_to_code(&self, tile: &TileType) -> String {
        match tile {
            TileType::Boundary => "X".to_string(),
            TileType::Grass => ".".to_string(),
            TileType::Soil => "#".to_string(),
            TileType::Crop(_, _) => "C".to_string(),
            TileType::House => "H".to_string(),
            TileType::PathEast => "P".to_string(),
            TileType::PathFarm => "P".to_string(),
            TileType::Player => "@".to_string(),
            TileType::Mushroom => "M".to_string(),
        }
    }

    fn tile_to_detailed_string(&self, tile: &TileType) -> String {
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
            TileType::Player => "Player".to_string(),
            TileType::Mushroom => "Mushroom".to_string(),
        }
    }

    pub fn to_inventory_snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "seeds": self.game_state.inventory.seeds,
            "produce": self.game_state.inventory.produce,
            "forage": self.game_state.inventory.forage,
            "money": self.game_state.money,
            "selected_seed": format!("{:?}", self.game_state.selected_seed),
        })
    }

    pub fn to_log_snapshot(&self, limit: Option<usize>) -> serde_json::Value {
        let max_entries = limit.unwrap_or(50);
        let recent_logs: Vec<&LogEntry> = self.logs.iter().rev().take(max_entries).collect();

        serde_json::json!({
            "total_entries": self.logs.len(),
            "entries": recent_logs
        })
    }

    /// Returns stats with minimum required fields: day, time, location, money, inventory, status.
    pub fn to_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "day": self.game_state.day,
            "time": self.game_state.format_time(),
            "location": format!("{:?}", self.game_state.location),
            "money": self.game_state.money,
            "inventory": {
                "seeds": self.game_state.inventory.seeds,
                "produce": self.game_state.inventory.produce,
                "forage": self.game_state.inventory.forage,
            },
            "status": if self.closed { "closed" } else { "ok" },
            "season": self.game_state.season,
            "weather": self.game_state.weather,
        })
    }
}

pub struct SessionManager {
    sessions: RwLock<HashMap<String, Arc<RwLock<Session>>>>,
    max_sessions: usize,
    idle_timeout_minutes: u64,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            max_sessions: 10,
            idle_timeout_minutes: 30,
        }
    }

    pub fn create_session(
        &self,
        seed: Option<u64>,
        mode: Option<String>,
    ) -> Result<Session, McpError> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| McpError::internal_error("Failed to acquire session lock"))?;

        if sessions.len() >= self.max_sessions {
            return Err(McpError::validation_error(
                "Max sessions reached",
                vec!["Close existing sessions before creating new ones"],
            ));
        }

        let session = Session::new(seed, mode);
        let session_id = session.id.clone();
        let session_clone = session.clone();
        sessions.insert(session_id, Arc::new(RwLock::new(session)));

        Ok(session_clone)
    }

    pub fn get_session(&self, session_id: &str) -> Result<Arc<RwLock<Session>>, McpError> {
        let sessions = self
            .sessions
            .read()
            .map_err(|_| McpError::internal_error("Failed to acquire session lock"))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| McpError::session_not_found(session_id))?
            .clone();

        let session_lock = session
            .read()
            .map_err(|_| McpError::internal_error("Failed to read session"))?;

        if session_lock.closed {
            return Err(McpError::session_closed(session_id));
        }

        drop(session_lock);

        if let Ok(mut s) = session.write() {
            s.last_accessed = chrono::Utc::now();
        }

        Ok(session)
    }

    pub fn close_session(&self, session_id: &str) -> Result<bool, McpError> {
        let sessions = self
            .sessions
            .write()
            .map_err(|_| McpError::internal_error("Failed to acquire session lock"))?;

        if let Some(session) = sessions.get(session_id) {
            let mut session = session
                .write()
                .map_err(|_| McpError::internal_error("Failed to write session"))?;
            session.closed = true;
            Ok(true)
        } else {
            Err(McpError::session_not_found(session_id))
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            game_state: self.game_state.clone(),
            created_at: self.created_at,
            last_accessed: self.last_accessed,
            closed: self.closed,
            logs: self.logs.clone(),
        }
    }
}
