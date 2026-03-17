use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;

use super::command::{execute_command, parse_command};
use super::errors::{ErrorCode, McpError};
use super::session::SessionManager;
use super::tools::{
    CommandBatchInput, CommandInput, EndSessionInput, GetMapInput, GetStateInput, GetStatsInput,
    StartSessionInput, StartSessionOutput,
};

static SESSION_MANAGER: std::sync::LazyLock<Arc<RwLock<SessionManager>>> =
    std::sync::LazyLock::new(|| Arc::new(RwLock::new(SessionManager::new())));

pub fn get_session_manager() -> Arc<RwLock<SessionManager>> {
    SESSION_MANAGER.clone()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

impl ToolResponse {
    pub fn success(result: serde_json::Value) -> Self {
        Self {
            ok: true,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(code: ErrorCode, message: String) -> Self {
        Self {
            ok: false,
            result: None,
            error: Some(McpError::new(code, message)),
        }
    }

    pub fn from_mcp_error(err: McpError) -> Self {
        Self {
            ok: false,
            result: None,
            error: Some(err),
        }
    }
}

pub fn handle_start_session(input: StartSessionInput) -> ToolResponse {
    let manager = get_session_manager();
    let manager = match manager.read() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.create_session(input.seed, input.mode) {
        Ok(session) => {
            let output = StartSessionOutput {
                session_id: session.id.clone(),
                initial_state: session.to_snapshot(),
            };
            ToolResponse::success(serde_json::to_value(output).unwrap_or_default())
        }
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

pub fn handle_end_session(input: EndSessionInput) -> ToolResponse {
    let manager = get_session_manager();
    let manager = match manager.write() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.close_session(&input.session_id) {
        Ok(_) => ToolResponse::success(serde_json::json!({ "ok": true })),
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

pub fn handle_get_state(input: GetStateInput) -> ToolResponse {
    let manager = get_session_manager();
    let manager = match manager.read() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.get_session(&input.session_id) {
        Ok(session) => {
            let session = match session.read() {
                Ok(s) => s,
                Err(_) => {
                    return ToolResponse::from_mcp_error(McpError::internal_error(
                        "Failed to read session",
                    ));
                }
            };
            ToolResponse::success(session.to_snapshot())
        }
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

pub fn handle_get_map(input: GetMapInput) -> ToolResponse {
    ToolResponse::error(
        ErrorCode::NotImplemented,
        "get_map not yet implemented".to_string(),
    )
}

pub fn handle_get_stats(input: GetStatsInput) -> ToolResponse {
    let manager = get_session_manager();
    let manager = match manager.read() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.get_session(&input.session_id) {
        Ok(session) => {
            let session = match session.read() {
                Ok(s) => s,
                Err(_) => {
                    return ToolResponse::from_mcp_error(McpError::internal_error(
                        "Failed to read session",
                    ));
                }
            };
            ToolResponse::success(session.to_snapshot())
        }
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

pub fn handle_command(input: CommandInput) -> ToolResponse {
    let manager = get_session_manager();
    let manager = match manager.read() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.get_session(&input.session_id) {
        Ok(session) => {
            let mut session = match session.write() {
                Ok(s) => s,
                Err(_) => {
                    return ToolResponse::from_mcp_error(McpError::internal_error(
                        "Failed to write session",
                    ));
                }
            };

            let parsed = match parse_command(&input.command) {
                Ok(cmd) => cmd,
                Err(e) => return ToolResponse::from_mcp_error(e),
            };

            let result = execute_command(&mut session.game_state, parsed);

            let result_json = serde_json::json!({
                "message": result.message,
                "events": result.events,
                "state_delta": result.state_delta,
                "snapshot_text": result.snapshot_text,
            });

            ToolResponse::success(result_json)
        }
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

pub fn handle_command_batch(input: CommandBatchInput) -> ToolResponse {
    if input.commands.is_empty() {
        return ToolResponse::error(
            ErrorCode::ValidationError,
            "commands array cannot be empty".to_string(),
        );
    }

    let manager = get_session_manager();
    let manager = match manager.read() {
        Ok(m) => m,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire session manager lock",
            ));
        }
    };

    match manager.get_session(&input.session_id) {
        Ok(session) => {
            let mut session = match session.write() {
                Ok(s) => s,
                Err(_) => {
                    return ToolResponse::from_mcp_error(McpError::internal_error(
                        "Failed to write session",
                    ));
                }
            };

            let mut results: Vec<serde_json::Value> = Vec::new();
            let mut executed_count: usize = 0;
            let mut last_error: Option<McpError> = None;

            for cmd_str in &input.commands {
                let parsed = match parse_command(cmd_str) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        last_error = Some(e.clone());
                        results.push(serde_json::json!({
                            "command": cmd_str,
                            "ok": false,
                            "error": {
                                "code": format!("{:?}", e.code),
                                "message": e.message,
                                "details": e.details,
                            }
                        }));
                        if input.stop_on_error {
                            break;
                        }
                        continue;
                    }
                };

                let result = execute_command(&mut session.game_state, parsed);

                results.push(serde_json::json!({
                    "command": cmd_str,
                    "ok": true,
                    "result": {
                        "message": result.message,
                        "events": result.events,
                        "state_delta": result.state_delta,
                        "snapshot_text": result.snapshot_text,
                    }
                }));

                executed_count += 1;
            }

            let final_state = session.to_snapshot();

            let result_json = serde_json::json!({
                "executed_count": executed_count,
                "results": results,
                "final_state": final_state,
                "stopped_early": last_error.is_some(),
            });

            ToolResponse::success(result_json)
        }
        Err(e) => ToolResponse::from_mcp_error(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_session_creates_session() {
        let input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let response = handle_start_session(input);

        assert!(response.ok);
        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        let output: StartSessionOutput = serde_json::from_value(result).unwrap();
        assert!(!output.session_id.is_empty());
    }

    #[test]
    fn test_start_session_with_seed() {
        let input = StartSessionInput {
            seed: Some(42),
            mode: None,
        };
        let response = handle_start_session(input);

        assert!(response.ok);
        let result = response.result.unwrap();
        let output: StartSessionOutput = serde_json::from_value(result).unwrap();
        assert!(!output.session_id.is_empty());
    }

    #[test]
    fn test_end_session_unknown_id() {
        let input = EndSessionInput {
            session_id: "non-existent-id".to_string(),
        };
        let response = handle_end_session(input);

        assert!(!response.ok);
        assert!(response.error.is_some());
        assert_eq!(
            response.error.as_ref().unwrap().code,
            ErrorCode::SessionNotFound
        );
    }

    #[test]
    fn test_get_state_unknown_session() {
        let input = GetStateInput {
            session_id: "non-existent-id".to_string(),
        };
        let response = handle_get_state(input);

        assert!(!response.ok);
        assert!(response.error.is_some());
        assert_eq!(
            response.error.as_ref().unwrap().code,
            ErrorCode::SessionNotFound
        );
    }

    #[test]
    fn test_session_lifecycle() {
        let start_input = StartSessionInput {
            seed: Some(123),
            mode: Some("standard".to_string()),
        };
        let start_response = handle_start_session(start_input);
        assert!(start_response.ok);

        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let state_input = GetStateInput {
            session_id: session_id.clone(),
        };
        let state_response = handle_get_state(state_input);
        assert!(state_response.ok);

        let end_input = EndSessionInput {
            session_id: session_id.clone(),
        };
        let end_response = handle_end_session(end_input);
        assert!(end_response.ok);

        let state_after_close = handle_get_state(GetStateInput { session_id });
        assert!(!state_after_close.ok);
        assert_eq!(
            state_after_close.error.as_ref().unwrap().code,
            ErrorCode::SessionClosed
        );
    }

    #[test]
    fn test_command_move() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let cmd_input = CommandInput {
            session_id: session_id.clone(),
            command: "move:down".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(response.ok);
        let result = response.result.unwrap();
        assert!(result.get("message").is_some());
        assert!(result.get("events").is_some());
    }

    #[test]
    fn test_command_print() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let cmd_input = CommandInput {
            session_id: session_id.clone(),
            command: "print".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(response.ok);
        let result = response.result.unwrap();
        assert!(result.get("snapshot_text").is_some());
        let snapshot = result.get("snapshot_text").unwrap().as_str().unwrap();
        assert!(snapshot.contains("Day"));
    }

    #[test]
    fn test_command_invalid() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let cmd_input = CommandInput {
            session_id: session_id.clone(),
            command: "fly:away".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(!response.ok);
        assert!(response.error.is_some());
        assert_eq!(
            response.error.as_ref().unwrap().code,
            ErrorCode::InvalidCommand
        );
    }

    #[test]
    fn test_command_unknown_session() {
        let cmd_input = CommandInput {
            session_id: "unknown-session".to_string(),
            command: "print".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(!response.ok);
        assert!(response.error.is_some());
        assert_eq!(
            response.error.as_ref().unwrap().code,
            ErrorCode::SessionNotFound
        );
    }

    #[test]
    fn test_command_batch() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let batch_input = CommandBatchInput {
            session_id: session_id.clone(),
            commands: vec![
                "move:down".to_string(),
                "move:down".to_string(),
                "print".to_string(),
            ],
            stop_on_error: true,
        };
        let response = handle_command_batch(batch_input);

        assert!(response.ok);
        let result = response.result.unwrap();
        assert_eq!(result.get("executed_count").unwrap(), 3);
        assert!(result.get("results").is_some());
        assert!(result.get("final_state").is_some());
    }

    #[test]
    fn test_command_batch_stop_on_error() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let batch_input = CommandBatchInput {
            session_id: session_id.clone(),
            commands: vec![
                "move:down".to_string(),
                "fly:away".to_string(),
                "move:up".to_string(),
            ],
            stop_on_error: true,
        };
        let response = handle_command_batch(batch_input);

        assert!(response.ok);
        let result = response.result.unwrap();
        assert_eq!(result.get("executed_count").unwrap(), 1);
        assert!(result.get("stopped_early").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_command_batch_empty() {
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id =
            serde_json::from_value::<StartSessionOutput>(start_response.result.unwrap())
                .unwrap()
                .session_id;

        let batch_input = CommandBatchInput {
            session_id: session_id.clone(),
            commands: vec![],
            stop_on_error: true,
        };
        let response = handle_command_batch(batch_input);

        assert!(!response.ok);
        assert!(response.error.is_some());
    }
}
