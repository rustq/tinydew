use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;

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
    ToolResponse::error(
        ErrorCode::NotImplemented,
        "command execution not yet implemented".to_string(),
    )
}

pub fn handle_command_batch(input: CommandBatchInput) -> ToolResponse {
    ToolResponse::error(
        ErrorCode::NotImplemented,
        "command_batch execution not yet implemented".to_string(),
    )
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
}
