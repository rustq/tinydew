# Shelldew MCP Support Specification

## 1) Objective

Define MCP (Model Context Protocol) support for Shelldew so external MCP clients/agents can play the game programmatically without interactive TUI keypresses.

This spec focuses on a local MCP server interface exposed by Shelldew.

---

## 2) Scope

### In Scope
- MCP server integration for Shelldew runtime
- Tool-style actions for core gameplay commands
- Read-only state/query endpoints for map, player, inventory, economy, and time
- Deterministic, structured responses suitable for automation
- Error model and minimal safety/validation rules

### Out of Scope (initial iteration)
- Multiplayer/session sharing
- Networked remote auth providers
- Save sync across machines/cloud
- Long-term replay export format

---

## 3) MCP Capabilities

Shelldew MCP server should expose:

1. **Tools** (actions/mutations)
2. **Resources** (read-only state snapshots)
3. **Optional prompts/templates** (helper prompts for automation agents)

Primary usage is via tools.

---

## 4) State Model

### 4.1 Singleton Game State
Shelldew uses a single persistent game state managed by the MCP server:

- Exactly one authoritative `GameState` instance exists at runtime.
- All tools read/write the same shared state.
- No per-session state isolation.
- State is automatically persisted when sleep/day transitions complete.

### 4.2 Session Compatibility (Soft-Compat)
For backwards compatibility, `startSession` and `endSession` methods are retained:

- `startSession` returns the constant session_id `"singleton"` and does not reset game state.
- `endSession` is a no-op success (game continues).
- All other tools accept any `session_id` string for compatibility.

### 4.3 Autosave Behavior
- Autosave triggers on day transition completion (sleep/income flow).
- Save destination: `~/.local/shelldew/savegame.json` (or `./savegame.json` fallback).
- If save fails, gameplay continues but response includes a warning.

### 4.4 Load-on-Start
- Server attempts to load latest save at startup.
- If save exists and valid: resume from saved state.
- If missing/corrupt: start new game (with warning logged).

### 4.5 Determinism
For testing automation:
- Support optional deterministic seed at game creation (`seed`)
- Document any RNG behavior affected by seed (e.g., forage spawn)

---

## 5) Tool API (Normative)

## 5.1 `shelldew.start_session`
Start or resume the singleton game session.

**Input**
- `seed` (optional integer) - used only on first game start
- `mode` (optional string, default `"standard"`)

**Output**
- `session_id` (string, always `"singleton"`)
- `initial_state` (object; minimal state schema)

---

## 5.2 `shelldew.command`
Execute one gameplay command.

**Input**
- `session_id` (string, required)
- `command` (string, required)

Supported command grammar:
- `move:up|down|left|right`
- `clear`
- `plant:<crop>` where crop in `carrot|strawberry|cauliflower|rhubarb`
- `water`
- `harvest`
- `buy:<item>[:<qty>]`
- `sell:<item>[:<qty>]`
- `sleep`
- `print` (returns text snapshot payload)

**Output**
- `ok` (boolean)
- `result` (object)
  - `message` (string)
  - `events` (array)
  - `state_delta` (object, optional)
  - `snapshot_text` (string, optional; for `print`)
- `error` (object|null)

---

## 5.3 `shelldew.command_batch`
Execute multiple commands in order.

**Input**
- `session_id` (string)
- `commands` (string array, required, non-empty)
- `stop_on_error` (boolean, default `true`)

**Output**
- `ok` (boolean)
- `executed_count` (integer)
- `results` (array of per-command result objects)
- `final_state` (object)

Behavior:
- Execute strictly in order.
- If `stop_on_error=true`, terminate on first error.

---

## 5.4 `shelldew.get_state`
Return current structured game state.

**Input**
- `session_id` (string)

**Output (minimal schema)**
- `day` (integer)
- `time` (`HH:MM` string)
- `location` (string)
- `money` (integer)
- `inventory` (object map)
- `player` (object: `x`, `y`)
- `status` (`ok|error`)

---

## 5.5 `shelldew.get_map`
Return current map view/state.

**Input**
- `session_id` (string)
- `include_entities` (boolean, default `true`)

**Output**
- `location` (string)
- `width` (integer)
- `height` (integer)
- `tiles` (2D array or row-string array)
- `legend` (object)

---

## 5.6 `shelldew.get_stats`
Return final/summary stats (aligned with non-interactive model).

**Input**
- `session_id` (string)

**Output**
- `day`
- `time`
- `location`
- `money`
- `inventory`
- `status`

---

## 5.7 `shelldew.end_session`
Gracefully close session and release resources.

**Input**
- `session_id` (string)

**Output**
- `ok` (boolean)

---

## 6) Resources (Read-Only)

Suggested MCP resources:
- `shelldew://session/{session_id}/state`
- `shelldew://session/{session_id}/map`
- `shelldew://session/{session_id}/inventory`
- `shelldew://session/{session_id}/log/recent`

Resources should mirror tool output schemas and be safe for frequent polling.

---

## 7) Validation Rules

- Unknown commands -> error code `INVALID_COMMAND`
- Invalid direction/crop/item/qty -> `VALIDATION_ERROR`
- Unknown session -> `SESSION_NOT_FOUND`
- Closed session use -> `SESSION_CLOSED`
- Quantity must be positive integer
- All mutation tools require valid `session_id`

---

## 8) Error Contract

All tool failures should return structured error object:

```json
{
  "code": "VALIDATION_ERROR",
  "message": "invalid crop 'tomato'",
  "details": {
    "valid": ["carrot", "strawberry", "cauliflower", "rhubarb"]
  }
}
```

Do not rely on unstructured text for machine control.

---

## 9) Observability

Minimum logging fields:
- timestamp
- session_id
- tool
- command (if applicable)
- success/failure
- duration_ms

Optional debug mode may include state deltas.

---

## 10) Security & Safety

- Local-only MCP transport by default
- No arbitrary shell execution via MCP
- Enforce command allowlist
- Single state instance with atomic save/load

Recommended settings (for future multi-user extension):
- idle timeout: 30 minutes
- max active sessions: 10

---

## 11) Compatibility Notes

MCP layer reuses Shelldew game logic used by:
- interactive TUI mode
- non-interactive command mode

Goal: identical gameplay semantics across all interfaces.

---

## 12) Acceptance Criteria

- MCP client can run a full farming loop without resetting state:
  - move -> clear -> plant -> water -> sleep -> harvest -> sell
- Repeated `startSession` calls do not reset progress.
- `endSession` does not destroy active game.
- Sleep transitions autosave state.
- Server restart resumes from last saved state.
- Structured state/stats retrievable at any point
- Batch command execution works deterministically with seed
- Error codes are stable and machine-readable

---

## 13) Future Extensions

- Prompt templates for autonomous farming goals
- Replay/event-stream resource
- Save/load tool APIs
- Multi-session orchestration helpers
