use crate::state::GameState;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use super::errors::McpError;

pub struct Session {
    pub id: String,
    pub game_state: GameState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub closed: bool,
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

        Self {
            id: Uuid::new_v4().to_string(),
            game_state,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            closed: false,
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
        }
    }
}
