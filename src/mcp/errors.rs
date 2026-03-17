use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidCommand,
    ValidationError,
    SessionNotFound,
    SessionClosed,
    InternalError,
    NotImplemented,
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

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_error_code_serialization() {
        let code = ErrorCode::InvalidCommand;
        let json = serde_json::to_string(&code).unwrap();
        assert!(json.contains("INVALID_COMMAND"));
    }

    #[test]
    fn test_error_code_deserialization() {
        let json = r#""INVALID_COMMAND""#;
        let code: ErrorCode = serde_json::from_str(json).unwrap();
        assert_eq!(code, ErrorCode::InvalidCommand);
    }

    #[test]
    fn test_all_error_codes() {
        let codes = vec![
            ErrorCode::InvalidCommand,
            ErrorCode::ValidationError,
            ErrorCode::SessionNotFound,
            ErrorCode::SessionClosed,
            ErrorCode::InternalError,
            ErrorCode::NotImplemented,
        ];

        for code in codes {
            let json = serde_json::to_string(&code).unwrap();
            let parsed: ErrorCode = serde_json::from_str(&json).unwrap();
            assert_eq!(code, parsed);
        }
    }

    #[test]
    fn test_mcp_error_new() {
        let error = McpError::new(ErrorCode::InvalidCommand, "test error");
        assert_eq!(error.code, ErrorCode::InvalidCommand);
        assert_eq!(error.message, "test error");
        assert!(error.details.is_none());
    }

    #[test]
    fn test_mcp_error_with_details() {
        let error = McpError::new(ErrorCode::ValidationError, "validation failed")
            .with_details(serde_json::json!({ "field": "value" }));
        assert!(error.details.is_some());
    }

    #[test]
    fn test_mcp_error_invalid_command() {
        let error = McpError::invalid_command("invalid");
        assert_eq!(error.code, ErrorCode::InvalidCommand);
    }

    #[test]
    fn test_mcp_error_validation_error() {
        let error = McpError::validation_error("invalid", vec!["a", "b"]);
        assert_eq!(error.code, ErrorCode::ValidationError);
        assert!(error.details.is_some());
        let details = error.details.unwrap();
        assert!(details.get("valid").is_some());
    }

    #[test]
    fn test_mcp_error_session_not_found() {
        let error = McpError::session_not_found("test-id");
        assert_eq!(error.code, ErrorCode::SessionNotFound);
        assert!(error.message.contains("test-id"));
    }

    #[test]
    fn test_mcp_error_session_closed() {
        let error = McpError::session_closed("test-id");
        assert_eq!(error.code, ErrorCode::SessionClosed);
        assert!(error.message.contains("test-id"));
    }

    #[test]
    fn test_mcp_error_internal_error() {
        let error = McpError::internal_error("internal error");
        assert_eq!(error.code, ErrorCode::InternalError);
    }

    #[test]
    fn test_mcp_error_serialization() {
        let error = McpError::new(ErrorCode::InvalidCommand, "test error");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("INVALID_COMMAND"));
        assert!(json.contains("test error"));
    }

    #[test]
    fn test_mcp_error_serialization_without_details() {
        let error = McpError::new(ErrorCode::InvalidCommand, "test error");
        let json = serde_json::to_string(&error).unwrap();
        assert!(!json.contains("details"));
    }

    #[test]
    fn test_mcp_error_serialization_with_details() {
        let error = McpError::new(ErrorCode::ValidationError, "test")
            .with_details(serde_json::json!({ "key": "value" }));
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("details"));
        assert!(json.contains("key"));
    }

    #[test]
    fn test_error_message_includes_code() {
        let error = McpError::new(ErrorCode::SessionNotFound, "session missing");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("SESSION_NOT_FOUND"));
    }
}
