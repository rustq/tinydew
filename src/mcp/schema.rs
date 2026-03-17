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

#[cfg(test)]
mod schema_tests {
    use super::*;

    #[test]
    fn test_game_state_snapshot_serialization() {
        let snapshot = GameStateSnapshot {
            day: 1,
            time: "08:00".to_string(),
            location: "Farm".to_string(),
            money: 100,
            inventory: serde_json::json!({ "seeds": {}, "produce": {} }),
            player: serde_json::json!({ "x": 0, "y": 0 }),
            status: "ok".to_string(),
        };

        let json = serde_json::to_string(&snapshot).unwrap();
        assert!(json.contains("\"day\":1"));
        assert!(json.contains("\"time\":\"08:00\""));
        assert!(json.contains("\"location\":\"Farm\""));
        assert!(json.contains("\"money\":100"));
        assert!(json.contains("\"status\":\"ok\""));
    }

    #[test]
    fn test_game_state_snapshot_deserialization() {
        let json = r#"{
            "day": 5,
            "time": "14:30",
            "location": "Farm",
            "money": 250,
            "inventory": {"seeds": {"carrot": 10}},
            "player": {"x": 3, "y": 2},
            "status": "ok"
        }"#;

        let snapshot: GameStateSnapshot = serde_json::from_str(json).unwrap();
        assert_eq!(snapshot.day, 5);
        assert_eq!(snapshot.time, "14:30");
        assert_eq!(snapshot.location, "Farm");
        assert_eq!(snapshot.money, 250);
        assert_eq!(snapshot.status, "ok");
    }

    #[test]
    fn test_request_log_serialization() {
        let log = RequestLog {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            tool: "start_session".to_string(),
            session_id: Some("abc123".to_string()),
            duration_ms: 50,
            success: true,
        };

        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("start_session"));
        assert!(json.contains("abc123"));
        assert!(json.contains("50"));
    }

    #[test]
    fn test_request_log_deserialization() {
        let json = r#"{
            "timestamp": "2024-01-01T12:00:00Z",
            "tool": "get_state",
            "session_id": "xyz789",
            "duration_ms": 25,
            "success": true
        }"#;

        let log: RequestLog = serde_json::from_str(json).unwrap();
        assert_eq!(log.tool, "get_state");
        assert_eq!(log.session_id, Some("xyz789".to_string()));
        assert_eq!(log.duration_ms, 25);
        assert!(log.success);
    }

    #[test]
    fn test_request_log_optional_session_id() {
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "tool": "list_tools",
            "session_id": null,
            "duration_ms": 5,
            "success": true
        }"#;

        let log: RequestLog = serde_json::from_str(json).unwrap();
        assert!(log.session_id.is_none());
    }

    #[test]
    fn test_roundtrip_game_state_snapshot() {
        let original = GameStateSnapshot {
            day: 3,
            time: "10:00".to_string(),
            location: "Farm".to_string(),
            money: 150,
            inventory: serde_json::json!({ "seeds": { "carrot": 5 }, "produce": {} }),
            player: serde_json::json!({ "x": 1, "y": 2 }),
            status: "ok".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: GameStateSnapshot = serde_json::from_str(&json).unwrap();

        assert_eq!(original.day, parsed.day);
        assert_eq!(original.time, parsed.time);
        assert_eq!(original.location, parsed.location);
        assert_eq!(original.money, parsed.money);
        assert_eq!(original.status, parsed.status);
    }

    #[test]
    fn test_roundtrip_request_log() {
        let original = RequestLog {
            timestamp: "2024-06-15T08:30:00Z".to_string(),
            tool: "command".to_string(),
            session_id: Some("session-123".to_string()),
            duration_ms: 100,
            success: false,
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RequestLog = serde_json::from_str(&json).unwrap();

        assert_eq!(original.timestamp, parsed.timestamp);
        assert_eq!(original.tool, parsed.tool);
        assert_eq!(original.session_id, parsed.session_id);
        assert_eq!(original.duration_ms, parsed.duration_ms);
        assert_eq!(original.success, parsed.success);
    }
}
