use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub day: u32,
    pub time: String,
    pub location: String,
    pub money: u32,
    pub inventory: serde_json::Value,
    pub player: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub timestamp: String,
    pub tool: String,
    pub session_id: Option<String>,
    pub duration_ms: u64,
    pub success: bool,
}
