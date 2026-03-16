use serde::{Deserialize, Serialize};
use tracing::info;

use super::resources::McpResources;
use super::tools::McpTools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub timestamp: String,
    pub tool: String,
    pub session_id: Option<String>,
    pub duration_ms: u64,
    pub success: bool,
}

pub fn initialize_mcp_server() {
    info!("Initializing MCP server 'shelldew'");
    info!("Registered tools:");
    for tool in McpTools::get_tool_definitions() {
        info!("  - {}: {}", tool.name, tool.description);
    }
    info!("Registered resources:");
    for resource in McpResources::get_resource_definitions() {
        info!("  - {} ({})", resource.uri, resource.mime_type);
    }
    info!("Transport: local-only (stdio)");
}

pub fn log_request(log: RequestLog) {
    let status = if log.success { "SUCCESS" } else { "FAILED" };
    info!(
        "[{}] tool={} session_id={:?} duration_ms={} status={}",
        log.timestamp, log.tool, log.session_id, log.duration_ms, status
    );
}

pub fn create_request_log(
    tool: &str,
    session_id: Option<String>,
    duration_ms: u64,
    success: bool,
) -> RequestLog {
    RequestLog {
        timestamp: chrono::Utc::now().to_rfc3339(),
        tool: tool.to_string(),
        session_id,
        duration_ms,
        success,
    }
}
