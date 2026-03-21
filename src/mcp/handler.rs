#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use super::command::{execute_command, parse_command};
use super::errors::{ErrorCode, McpError};
use super::state_manager::{
    GameStateManager, SINGLETON_SESSION_ID, SharedGameState, create_shared_state,
};
use super::tools::{
    CommandBatchInput, CommandInput, EndSessionInput, GetMapInput, GetStateInput, GetStatsInput,
    GetWorldTimeInput, StartSessionInput, StartSessionOutput,
};

static GAME_STATE: LazyLock<SharedGameState> = LazyLock::new(create_shared_state);

use std::sync::LazyLock;

pub fn get_game_state() -> SharedGameState {
    GAME_STATE.clone()
}

#[cfg(test)]
pub fn reset_for_tests() {
    let mut state = GAME_STATE.lock().unwrap();
    *state = GameStateManager::new();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub code: String,
    pub message: String,
}

impl ToolResponse {
    pub fn success(result: serde_json::Value) -> Self {
        Self {
            ok: true,
            result: Some(result),
            error: None,
            warnings: None,
        }
    }

    pub fn success_with_warnings(result: serde_json::Value, warnings: Vec<Warning>) -> Self {
        Self {
            ok: true,
            result: Some(result),
            error: None,
            warnings: Some(warnings),
        }
    }

    pub fn error(code: ErrorCode, message: String) -> Self {
        Self {
            ok: false,
            result: None,
            error: Some(McpError::new(code, message)),
            warnings: None,
        }
    }

    pub fn from_mcp_error(err: McpError) -> Self {
        Self {
            ok: false,
            result: None,
            error: Some(err),
            warnings: None,
        }
    }
}

fn validate_session_id_compat(session_id: &str) -> Result<(), McpError> {
    if session_id.is_empty() {
        return Ok(());
    }
    if session_id.len() > 256 {
        return Err(McpError::validation_error(
            "session_id too long",
            vec!["Session ID must be 256 characters or less"],
        ));
    }
    if session_id != SINGLETON_SESSION_ID {
        return Ok(());
    }
    Ok(())
}

fn get_state_snapshot(state: &GameStateManager) -> serde_json::Value {
    state.to_snapshot()
}

fn autosave_if_needed(manager: &mut GameStateManager) -> Option<Warning> {
    if let Err(e) = manager.save() {
        Some(Warning {
            code: "SAVE_FAILED".to_string(),
            message: format!("Autosave failed: {}", e),
        })
    } else {
        None
    }
}

fn should_autosave_after_command(cmd: &super::command::ParsedCommand) -> bool {
    use super::command::ParsedCommand;
    matches!(
        cmd,
        ParsedCommand::Move(_)
            | ParsedCommand::Clear(_)
            | ParsedCommand::Plant(_, _)
            | ParsedCommand::Water(_)
            | ParsedCommand::Harvest(_)
            | ParsedCommand::BuySeed(_)
            | ParsedCommand::Sell(_, _)
    )
}


pub fn handle_start_session(_input: StartSessionInput) -> ToolResponse {
    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let output = StartSessionOutput {
        session_id: SINGLETON_SESSION_ID.to_string(),
        initial_state: state.to_snapshot(),
    };
    ToolResponse::success(serde_json::to_value(output).unwrap_or_default())
}

pub fn handle_end_session(input: EndSessionInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);
    ToolResponse::success(serde_json::json!({ "ok": true }))
}

pub fn handle_get_state(input: GetStateInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);

    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    ToolResponse::success(state.to_snapshot())
}

pub fn handle_get_map(input: GetMapInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);

    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let include_entities = input.include_entities.unwrap_or(false);
    let map_data = state.to_map_snapshot(include_entities);
    ToolResponse::success(map_data)
}

pub fn handle_get_stats(input: GetStatsInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);

    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let stats = state.to_stats();
    ToolResponse::success(stats)
}

pub fn handle_get_world_time(input: GetWorldTimeInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);

    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let (day, hour, minute) = state.get_day_and_time();
    let result = serde_json::json!({
        "hour": hour,
        "minute": minute,
        "day": day,
    });
    ToolResponse::success(result)
}

pub fn handle_command(input: CommandInput) -> ToolResponse {
    let _ = validate_session_id_compat(&input.session_id);

    let mut state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let parsed = match parse_command(&input.command) {
        Ok(cmd) => cmd,
        Err(e) => return ToolResponse::from_mcp_error(e),
    };

    let should_autosave = should_autosave_after_command(&parsed);
    let result = execute_command(&mut state.state, parsed);
    state.mark_dirty();

    let mut warnings: Vec<Warning> = Vec::new();

    if should_autosave {
        if let Some(warning) = autosave_if_needed(&mut state) {
            warnings.push(warning);
        }
    }

    let result_json = serde_json::json!({
        "message": result.message,
        "events": result.events,
        "state_delta": result.state_delta,
        "snapshot_text": result.snapshot_text,
    });

    if warnings.is_empty() {
        ToolResponse::success(result_json)
    } else {
        ToolResponse::success_with_warnings(result_json, warnings)
    }
}

pub fn handle_command_batch(input: CommandBatchInput) -> ToolResponse {
    if input.commands.is_empty() {
        return ToolResponse::error(
            ErrorCode::ValidationError,
            "commands array cannot be empty".to_string(),
        );
    }

    let _ = validate_session_id_compat(&input.session_id);

    let mut state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let mut results: Vec<serde_json::Value> = Vec::new();
    let mut executed_count: usize = 0;
    let mut last_error: Option<McpError> = None;
    let mut autosaved = false;

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


        let result = execute_command(&mut state.state, parsed);

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

    state.mark_dirty();

    let mut warnings: Vec<Warning> = Vec::new();

    if state.dirty {
        if let Some(warning) = autosave_if_needed(&mut state) {
            warnings.push(warning);
        }
        autosaved = true;
    }

    let final_state = state.to_snapshot();

    let mut result_json = serde_json::json!({
        "executed_count": executed_count,
        "results": results,
        "final_state": final_state,
        "stopped_early": last_error.is_some(),
    });

    if autosaved {
        result_json["autosaved"] = serde_json::json!(true);
        result_json["save_timestamp"] =
            serde_json::json!(state.last_save_time.map(|t| t.to_rfc3339()));
    }

    if warnings.is_empty() {
        ToolResponse::success(result_json)
    } else {
        ToolResponse::success_with_warnings(result_json, warnings)
    }
}

pub fn handle_resource_read(uri: &str) -> ToolResponse {
    use super::resources::McpResources;

    let parsed = match McpResources::parse_resource_uri(uri) {
        Some(p) => p,
        None => {
            return ToolResponse::error(
                ErrorCode::ValidationError,
                format!("Invalid resource URI: {}", uri),
            );
        }
    };

    let (session_id, resource_type) = parsed;

    let _ = validate_session_id_compat(&session_id);

    let state = match GAME_STATE.lock() {
        Ok(s) => s,
        Err(_) => {
            return ToolResponse::from_mcp_error(McpError::internal_error(
                "Failed to acquire state lock",
            ));
        }
    };

    let result = match resource_type.as_str() {
        "state" => state.to_snapshot(),
        "map" => state.to_map_snapshot(false),
        "inventory" => state.to_inventory_snapshot(),
        _ => {
            return ToolResponse::error(
                ErrorCode::ValidationError,
                format!("Unknown resource type: {}", resource_type),
            );
        }
    };

    ToolResponse::success(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_session_returns_singleton() {
        reset_for_tests();
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
        assert_eq!(output.session_id, SINGLETON_SESSION_ID);
    }

    #[test]
    fn test_start_session_repeated_no_reset() {
        reset_for_tests();
        let mut state = GAME_STATE.lock().unwrap();
        state.state.money = 1000;
        state
            .state
            .inventory
            .seeds
            .insert(crate::world::CropType::Carrot, 5);
        drop(state);

        let input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let response = handle_start_session(input);

        assert!(response.ok);
        let result = response.result.unwrap();
        let output: StartSessionOutput = serde_json::from_value(result).unwrap();

        let money = output.initial_state.get("money").unwrap().as_u64().unwrap();
        assert_eq!(money, 1000);

        let seeds = output
            .initial_state
            .get("inventory")
            .unwrap()
            .get("seeds")
            .unwrap();
        assert!(seeds.get("Carrot").is_some());
    }

    #[test]
    fn test_end_session_noop() {
        let input = EndSessionInput {
            session_id: "any-session-id".to_string(),
        };
        let response = handle_end_session(input);

        assert!(response.ok);
        assert!(response.error.is_none());

        let state = handle_get_state(GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        });
        assert!(state.ok);
    }

    #[test]
    fn test_end_session_empty_session_id() {
        let input = EndSessionInput {
            session_id: "".to_string(),
        };
        let response = handle_end_session(input);

        assert!(response.ok);
    }

    #[test]
    fn test_command_move() {
        reset_for_tests();
        let cmd_input = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
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
        reset_for_tests();
        let cmd_input = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "print".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(response.ok);
        let result = response.result.unwrap();
        assert!(result.get("snapshot_text").is_some());
        let snapshot = result.get("snapshot_text").unwrap().as_str().unwrap();
        assert!(snapshot.contains("tinydew day"));
    }

    #[test]
    fn test_command_invalid() {
        reset_for_tests();
        let cmd_input = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
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
    fn test_command_empty_session_id() {
        reset_for_tests();
        let cmd_input = CommandInput {
            session_id: "".to_string(),
            command: "print".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(response.ok);
    }

    #[test]
    fn test_command_arbitrary_session_id() {
        reset_for_tests();
        let cmd_input = CommandInput {
            session_id: "random-session-123".to_string(),
            command: "print".to_string(),
        };
        let response = handle_command(cmd_input);

        assert!(response.ok);
    }

    #[test]
    fn test_command_batch() {
        reset_for_tests();
        let batch_input = CommandBatchInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
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
        assert_eq!(result.get("executed_count").unwrap().as_u64().unwrap(), 3);
        assert!(result.get("results").is_some());
        assert!(result.get("final_state").is_some());
    }

    #[test]
    fn test_command_batch_stop_on_error() {
        reset_for_tests();
        let batch_input = CommandBatchInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
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
        assert_eq!(result.get("executed_count").unwrap().as_u64().unwrap(), 1);
        assert!(result.get("stopped_early").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_command_batch_empty() {
        reset_for_tests();
        let batch_input = CommandBatchInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            commands: vec![],
            stop_on_error: true,
        };
        let response = handle_command_batch(batch_input);

        assert!(!response.ok);
        assert!(response.error.is_some());
    }

    #[test]
    fn test_full_lifecycle() {
        reset_for_tests();
        let start_input = StartSessionInput {
            seed: None,
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        assert!(start_response.ok);

        let cmd_input = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "move:down".to_string(),
        };
        let cmd_response = handle_command(cmd_input);
        assert!(cmd_response.ok);

        let state_input = GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        };
        let state_response = handle_get_state(state_input);
        assert!(state_response.ok);

        let map_input = GetMapInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            include_entities: Some(true),
        };
        let map_response = handle_get_map(map_input);
        assert!(map_response.ok);

        let stats_input = GetStatsInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        };
        let stats_response = handle_get_stats(stats_input);
        assert!(stats_response.ok);

        let end_input = EndSessionInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        };
        let end_response = handle_end_session(end_input);
        assert!(end_response.ok);

        let state_after = handle_get_state(GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        });
        assert!(state_after.ok);
    }

    #[test]
    fn test_seed_determinism() {
        reset_for_tests();
        let mut state = GAME_STATE.lock().unwrap();
        state.state.day = 1;
        state.state.money = 500;
        drop(state);

        let cmd1 = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "print".to_string(),
        };
        let print1 = handle_command(cmd1);
        let result1 = print1.result.unwrap();
        let snapshot1 = result1
            .get("snapshot_text")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        reset_for_tests();
        let mut state = GAME_STATE.lock().unwrap();
        state.state.day = 1;
        state.state.money = 500;
        drop(state);

        let cmd2 = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "print".to_string(),
        };
        let print2 = handle_command(cmd2);
        let result2 = print2.result.unwrap();
        let snapshot2 = result2
            .get("snapshot_text")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        assert_eq!(snapshot1, snapshot2);
    }

    #[test]
    fn test_after_midnight_no_forced_sleep_cycle() {
        reset_for_tests();

        let state_input = GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        };
        let state_response = handle_get_state(state_input.clone());
        let initial_state = state_response.result.unwrap();
        assert_eq!(
            initial_state.get("time").unwrap().as_str().unwrap(),
            "06:00"
        );
        assert_eq!(initial_state.get("day").unwrap().as_u64().unwrap(), 1);

        let directions = ["move:down", "move:right", "move:up", "move:left"];
        let commands: Vec<String> = (0..240).map(|i| directions[i % 4].to_string()).collect();

        let batch_input = CommandBatchInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            commands,
            stop_on_error: true,
        };
        let batch_resp = handle_command_batch(batch_input);
        assert!(batch_resp.ok);

        let state_input = GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        };
        let state_response = handle_get_state(state_input);
        let state = state_response.result.unwrap();

        let time_str = state.get("time").unwrap().as_str().unwrap();
        let day_val = state.get("day").unwrap().as_u64().unwrap();

        assert_eq!(time_str, "02:00");
        assert_eq!(day_val, 2);

        let print_resp = handle_command(CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "print".to_string(),
        });
        assert!(print_resp.ok);
        let print_result = print_resp.result.unwrap();
        let snapshot = print_result
            .get("snapshot_text")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        assert!(snapshot.contains("It's after midnight. You should go back home and sleep."));
    }

    #[test]
    fn test_tool_response_success() {
        let response = ToolResponse::success(serde_json::json!({"key": "value"}));
        assert!(response.ok);
        assert!(response.error.is_none());
        assert!(response.result.is_some());
        assert!(response.warnings.is_none());
    }

    #[test]
    fn test_tool_response_error() {
        let response = ToolResponse::error(ErrorCode::InvalidCommand, "test error".to_string());
        assert!(!response.ok);
        assert!(response.error.is_some());
        assert!(response.result.is_none());
    }

    #[test]
    fn test_tool_response_from_mcp_error() {
        let mcp_error = McpError::invalid_command("test error");
        let response = ToolResponse::from_mcp_error(mcp_error);
        assert!(!response.ok);
        assert!(response.error.is_some());
    }

    #[test]
    fn test_singleton_session_id_constant() {
        assert_eq!(SINGLETON_SESSION_ID, "singleton");
    }

    #[test]
    fn test_validate_session_id_compat_empty() {
        let result = validate_session_id_compat("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_session_id_compat_singleton() {
        let result = validate_session_id_compat(SINGLETON_SESSION_ID);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_session_id_compat_arbitrary() {
        let result = validate_session_id_compat("some-random-id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_session_id_compat_too_long() {
        let long_id = "a".repeat(300);
        let result = validate_session_id_compat(&long_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_sleep_command_in_mcp_api_wakes_to_0600() {
        use crate::mcp::state_manager::SINGLETON_SESSION_ID;

        reset_for_tests();

        let cmd_sleep = CommandInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            command: "sleep".to_string(),
        };
        let response = handle_command(cmd_sleep);

        assert!(response.ok);

        let state = handle_get_state(GetStateInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
        })
        .result
        .unwrap();

        assert_eq!(state.get("time").and_then(|v| v.as_str()), Some("06:00"));
    }

    #[test]
    fn test_sleep_in_batch_is_allowed() {
        use crate::mcp::state_manager::SINGLETON_SESSION_ID;

        reset_for_tests();

        let batch = CommandBatchInput {
            session_id: SINGLETON_SESSION_ID.to_string(),
            commands: vec!["sleep".to_string()],
            stop_on_error: true,
        };

        let response = handle_command_batch(batch);
        assert!(response.ok);

        let result = response.result.unwrap();
        let first = result
            .get("results")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned()
            .unwrap_or_default();

        assert_eq!(first.get("ok").and_then(|v| v.as_bool()), Some(true));
    }
}
