# MCP Task 03: Command Tools (`command` + `command_batch`)

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 3)

## Goal
Enable MCP clients to execute Shelldew gameplay commands with deterministic, structured results.

## Todo
- [ ] Implement `shelldew.command` with required inputs:
  - [ ] `session_id`
  - [ ] `command`
- [ ] Support command grammar:
  - [ ] `move:up|down|left|right`
  - [ ] `clear`
  - [ ] `plant:<crop>`
  - [ ] `water`
  - [ ] `harvest`
  - [ ] `buy:<item>[:<qty>]`
  - [ ] `sell:<item>[:<qty>]`
  - [ ] `sleep`
  - [ ] `print`
- [ ] Implement `shelldew.command_batch`:
  - [ ] ordered execution
  - [ ] `stop_on_error` behavior
  - [ ] `executed_count` and per-command result list
- [ ] Return structured response object:
  - [ ] `ok`
  - [ ] `result.message`
  - [ ] `result.events`
  - [ ] optional `result.state_delta`
  - [ ] optional `result.snapshot_text` (for `print`)

## Acceptance
- [ ] Full farming command flow works via MCP tools.
- [ ] Batch mode executes in order and stops correctly on first error when configured.
- [ ] Validation failures are structured and machine-readable.

## Notes
- Reuse existing Shelldew command/gameplay logic to avoid semantic drift from TUI/non-interactive mode.