#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    pub uri: String,
    pub name: String,
    pub mime_type: String,
}

pub struct McpResources;

impl McpResources {
    pub fn get_resource_definitions() -> Vec<ResourceDefinition> {
        vec![
            ResourceDefinition {
                uri: "shelldew://session/{session_id}/state".to_string(),
                name: "Game state snapshot for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "shelldew://session/{session_id}/map".to_string(),
                name: "Current map view for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "shelldew://session/{session_id}/inventory".to_string(),
                name: "Player inventory for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "shelldew://session/{session_id}/log/recent".to_string(),
                name: "Recent log entries for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
        ]
    }
}
