use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidCommand,
    ValidationError,
    SessionNotFound,
    SessionClosed,
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl McpError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn invalid_command(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidCommand, msg)
    }

    pub fn validation_error(msg: impl Into<String>, valid: Vec<&str>) -> Self {
        Self::new(ErrorCode::ValidationError, msg)
            .with_details(serde_json::json!({ "valid": valid }))
    }

    pub fn session_not_found(session_id: &str) -> Self {
        Self::new(
            ErrorCode::SessionNotFound,
            format!("Session not found: {}", session_id),
        )
    }

    pub fn session_closed(session_id: &str) -> Self {
        Self::new(
            ErrorCode::SessionClosed,
            format!("Session closed: {}", session_id),
        )
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, msg)
    }
}
