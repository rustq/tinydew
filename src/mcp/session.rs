#![allow(dead_code)]
#![allow(deprecated)]

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

#[derive(Debug)]
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
            "weather": format!("{:?}", self.game_state.weather),
            "weather_day": self.game_state.weather_day,
            "is_paused": self.game_state.is_paused,
            "location": format!("{:?}", self.game_state.location),
            "money": self.game_state.money,
            "message": self.game_state.message,
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
    ///
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
            TileType::Piano => "N".to_string(),
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
            TileType::Piano => "Piano".to_string(),
        }
    }

    pub fn to_inventory_snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "seeds": self.game_state.inventory.seeds,
            "produce": self.game_state.inventory.produce,
            "forage": self.game_state.inventory.forage,
            "fish": self.game_state.inventory.fish,
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
            "weather": format!("{:?}", self.game_state.weather),
            "heartbeat": {
                "is_paused": self.game_state.is_paused,
            }
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

    pub fn cleanup_idle_sessions(&self) -> usize {
        let now = chrono::Utc::now();
        let timeout = chrono::Duration::minutes(self.idle_timeout_minutes as i64);

        let mut sessions = match self.sessions.write() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let idle_session_ids: Vec<String> = sessions
            .iter()
            .filter_map(|(id, session)| {
                let session = session.read().ok()?;
                if session.closed || now.signed_duration_since(session.last_accessed) > timeout {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        for id in &idle_session_ids {
            sessions.remove(id);
        }

        idle_session_ids.len()
    }

    pub fn get_active_session_count(&self) -> usize {
        let sessions = match self.sessions.read() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        sessions
            .values()
            .filter_map(|s| s.read().ok())
            .filter(|s| !s.closed)
            .count()
    }

    pub fn get_max_sessions(&self) -> usize {
        self.max_sessions
    }

    pub fn get_idle_timeout_minutes(&self) -> u64 {
        self.idle_timeout_minutes
    }

    #[cfg(test)]
    pub fn clear_all(&self) {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.clear();
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

#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new(None, None);
        assert!(!session.id.is_empty());
        assert!(!session.closed);
        assert_eq!(session.logs.len(), 1);
        assert_eq!(session.logs[0].action, "session_start");
    }

    #[test]
    fn test_session_creation_with_seed() {
        let session = Session::new(Some(42), None);
        assert!(!session.id.is_empty());
    }

    #[test]
    fn test_session_creation_with_mode() {
        let session = Session::new(None, Some("test".to_string()));
        assert!(!session.id.is_empty());
        assert!(session.game_state.message.contains("test"));
    }

    #[test]
    fn test_session_to_snapshot() {
        let session = Session::new(None, None);
        let snapshot = session.to_snapshot();
        assert!(snapshot.get("day").is_some());
        assert!(snapshot.get("time").is_some());
        assert!(snapshot.get("location").is_some());
        assert!(snapshot.get("money").is_some());
        assert!(snapshot.get("inventory").is_some());
        assert!(snapshot.get("player").is_some());
        assert_eq!(snapshot.get("status").unwrap(), "ok");
    }

    #[test]
    fn test_session_to_snapshot_closed() {
        let mut session = Session::new(None, None);
        session.closed = true;
        let snapshot = session.to_snapshot();
        assert_eq!(snapshot.get("status").unwrap(), "closed");
    }

    #[test]
    fn test_session_to_map_snapshot() {
        let session = Session::new(None, None);
        let map = session.to_map_snapshot(false);
        assert!(map.get("location").is_some());
        assert!(map.get("width").is_some());
        assert!(map.get("height").is_some());
        assert!(map.get("player_x").is_some());
        assert!(map.get("player_y").is_some());
        assert!(map.get("tiles").is_some());
    }

    #[test]
    fn test_session_to_map_snapshot_with_entities() {
        let session = Session::new(None, None);
        let map = session.to_map_snapshot(true);
        let tiles = map.get("tiles").unwrap().as_array().unwrap();
        for row in tiles {
            for tile in row.as_array().unwrap() {
                let tile_str = tile.as_str().unwrap();
                assert!(tile_str.len() > 1);
            }
        }
    }

    #[test]
    fn test_session_to_inventory_snapshot() {
        let session = Session::new(None, None);
        let inv = session.to_inventory_snapshot();
        assert!(inv.get("seeds").is_some());
        assert!(inv.get("produce").is_some());
        assert!(inv.get("forage").is_some());
        assert!(inv.get("money").is_some());
        assert!(inv.get("selected_seed").is_some());
    }

    #[test]
    fn test_session_to_log_snapshot() {
        let session = Session::new(None, None);
        let log = session.to_log_snapshot(None);
        assert_eq!(log.get("total_entries").unwrap().as_u64().unwrap(), 1);
        assert!(log.get("entries").is_some());
    }

    #[test]
    fn test_session_to_log_snapshot_with_limit() {
        let mut session = Session::new(None, None);
        session.logs.push(LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            day: 1,
            time: "08:00".to_string(),
            action: "test".to_string(),
            result: "test result".to_string(),
        });
        let log = session.to_log_snapshot(Some(1));
        assert_eq!(log.get("total_entries").unwrap().as_u64().unwrap(), 2);
    }

    #[test]
    fn test_session_to_stats() {
        let session = Session::new(None, None);
        let stats = session.to_stats();
        assert!(stats.get("day").is_some());
        assert!(stats.get("time").is_some());
        assert!(stats.get("location").is_some());
        assert!(stats.get("money").is_some());
        assert!(stats.get("inventory").is_some());
        assert!(stats.get("status").is_some());
        assert!(stats.get("season").is_some());
        assert!(stats.get("weather").is_some());
    }
}

#[cfg(test)]
mod session_manager_tests {
    use super::*;
    use crate::mcp::errors::ErrorCode;

    #[test]
    fn test_session_manager_new() {
        let manager = SessionManager::new();
        assert_eq!(manager.get_active_session_count(), 0);
        assert_eq!(manager.get_max_sessions(), 10);
        assert_eq!(manager.get_idle_timeout_minutes(), 30);
    }

    #[test]
    fn test_create_session() {
        let manager = SessionManager::new();
        let result = manager.create_session(None, None);
        assert!(result.is_ok());
        let session = result.unwrap();
        assert!(!session.id.is_empty());
        assert_eq!(manager.get_active_session_count(), 1);
    }

    #[test]
    fn test_create_session_with_seed_and_mode() {
        let manager = SessionManager::new();
        let result = manager.create_session(Some(42), Some("test".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_max_sessions() {
        let manager = SessionManager::new();
        let mut session_ids = Vec::new();

        for i in 0..10 {
            let result = manager.create_session(None, None);
            assert!(result.is_ok(), "Failed at iteration {}", i);
            session_ids.push(result.unwrap().id);
        }

        let result = manager.create_session(None, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::ValidationError);

        for id in session_ids {
            let _ = manager.close_session(&id);
        }
    }

    #[test]
    fn test_get_session() {
        let manager = SessionManager::new();
        let created = manager.create_session(None, None).unwrap();
        let session = manager.get_session(&created.id);
        assert!(session.is_ok());
    }

    #[test]
    fn test_get_session_not_found() {
        let manager = SessionManager::new();
        let session = manager.get_session("non-existent");
        assert!(session.is_err());
        let err = session.unwrap_err();
        assert_eq!(err.code, ErrorCode::SessionNotFound);
    }

    #[test]
    fn test_get_session_closed() {
        let manager = SessionManager::new();
        let created = manager.create_session(None, None).unwrap();
        manager.close_session(&created.id).unwrap();

        let session = manager.get_session(&created.id);
        assert!(session.is_err());
        let err = session.unwrap_err();
        assert_eq!(err.code, ErrorCode::SessionClosed);
    }

    #[test]
    fn test_close_session() {
        let manager = SessionManager::new();
        let created = manager.create_session(None, None).unwrap();
        let result = manager.close_session(&created.id);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_close_session_not_found() {
        let manager = SessionManager::new();
        let result = manager.close_session("non-existent");
        assert!(result.is_err());
    }

    #[test]
    fn test_cleanup_idle_sessions() {
        let manager = SessionManager::new();
        let created = manager.create_session(None, None).unwrap();
        manager.close_session(&created.id).unwrap();

        let cleaned = manager.cleanup_idle_sessions();
        assert!(cleaned >= 1);
    }

    #[test]
    fn test_get_active_session_count() {
        let manager = SessionManager::new();
        assert_eq!(manager.get_active_session_count(), 0);

        let created = manager.create_session(None, None).unwrap();
        assert_eq!(manager.get_active_session_count(), 1);

        manager.close_session(&created.id).unwrap();
        assert_eq!(manager.get_active_session_count(), 0);
    }

    #[test]
    fn test_multiple_sessions() {
        let manager = SessionManager::new();

        let session1 = manager.create_session(None, None).unwrap();
        let session2 = manager.create_session(None, None).unwrap();
        let session3 = manager.create_session(None, None).unwrap();

        assert_ne!(session1.id, session2.id);
        assert_ne!(session2.id, session3.id);
        assert_ne!(session1.id, session3.id);

        assert_eq!(manager.get_active_session_count(), 3);
    }
}
