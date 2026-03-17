# MCP Task 03: Command Tools (`command` + `command_batch`)

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 3)

## Goal
Enable MCP clients to execute Shelldew gameplay commands with deterministic, structured results.

## Todo
- [x] Implement `shelldew.command` with required inputs:
  - [x] `session_id`
  - [x] `command`
- [x] Support command grammar:
  - [x] `move:up|down|left|right`
  - [x] `clear`
  - [x] `plant:<crop>`
  - [x] `water`
  - [x] `harvest`
  - [x] `buy:<item>[:<qty>]`
  - [x] `sell:<item>[:<qty>]`
  - [x] `sleep`
  - [x] `print`
- [x] Implement `shelldew.command_batch`:
  - [x] ordered execution
  - [x] `stop_on_error` behavior
  - [x] `executed_count` and per-command result list
- [x] Return structured response object:
  - [x] `ok`
  - [x] `result.message`
  - [x] `result.events`
  - [x] optional `result.state_delta`
  - [x] optional `result.snapshot_text` (for `print`)

## Acceptance
- [x] Full farming command flow works via MCP tools.
- [x] Batch mode executes in order and stops correctly on first error when configured.
- [x] Validation failures are structured and machine-readable.

## Notes
- Reuse existing Shelldew command/gameplay logic to avoid semantic drift from TUI/non-interactive mode.

## Completed
- Created `src/mcp/command.rs` with command parsing and execution
- Implemented `handle_command` in `src/mcp/handler.rs` supporting all grammar
- Implemented `handle_command_batch` with ordered execution and stop_on_error
- Added comprehensive tests for command parsing and execution
- All 49 tests pass, cargo fmt and clippy pass