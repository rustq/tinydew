#![allow(dead_code)]

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
                uri: "tinydew://session/{session_id}/state".to_string(),
                name: "Game state snapshot for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "tinydew://session/{session_id}/map".to_string(),
                name: "Current map view for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "tinydew://session/{session_id}/inventory".to_string(),
                name: "Player inventory for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
            ResourceDefinition {
                uri: "tinydew://session/{session_id}/log/recent".to_string(),
                name: "Recent log entries for a session".to_string(),
                mime_type: "application/json".to_string(),
            },
        ]
    }

    pub fn parse_resource_uri(uri: &str) -> Option<(String, String)> {
        let prefix = "tinydew://session/";
        if let Some(rest) = uri.strip_prefix(prefix) {
            let parts: Vec<&str> = rest.split('/').collect();
            if parts.len() >= 2 {
                let session_id = parts[0].to_string();
                let resource_type = parts[1..].join("/");
                return Some((session_id, resource_type));
            }
        }
        None
    }
}
