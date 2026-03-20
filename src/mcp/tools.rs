use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartSessionInput {
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartSessionOutput {
    pub session_id: String,
    pub initial_state: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInput {
    pub session_id: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBatchInput {
    pub session_id: String,
    pub commands: Vec<String>,
    #[serde(default = "default_true")]
    pub stop_on_error: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStateInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMapInput {
    pub session_id: String,
    #[serde(default)]
    pub include_entities: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatsInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorldTimeInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndSessionInput {
    pub session_id: String,
}

pub struct McpTools;

impl McpTools {
    pub fn get_tool_definitions() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "tinydew.start_session".to_string(),
                description: "Create a new game session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "seed": {
                            "type": "integer",
                            "description": "Optional deterministic seed for session"
                        },
                        "mode": {
                            "type": "string",
                            "description": "Game mode (default: standard)"
                        }
                    }
                }),
            },
            ToolDefinition {
                name: "tinydew.command".to_string(),
                description: "Execute one gameplay command".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" },
                        "command": { "type": "string" }
                    },
                    "required": ["session_id", "command"]
                }),
            },
            ToolDefinition {
                name: "tinydew.command_batch".to_string(),
                description: "Execute multiple commands in order".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" },
                        "commands": { "type": "array", "items": { "type": "string" } },
                        "stop_on_error": { "type": "boolean" }
                    },
                    "required": ["session_id", "commands"]
                }),
            },
            ToolDefinition {
                name: "tinydew.get_state".to_string(),
                description: "Return current structured game state".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" }
                    },
                    "required": ["session_id"]
                }),
            },
            ToolDefinition {
                name: "tinydew.get_map".to_string(),
                description: "Return current map view/state".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" },
                        "include_entities": { "type": "boolean" }
                    },
                    "required": ["session_id"]
                }),
            },
            ToolDefinition {
                name: "tinydew.get_stats".to_string(),
                description: "Return final/summary stats".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" }
                    },
                    "required": ["session_id"]
                }),
            },
            ToolDefinition {
                name: "tinydew.end_session".to_string(),
                description: "Gracefully close session and release resources".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": { "type": "string" }
                    },
                    "required": ["session_id"]
                }),
            },
        ]
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}
