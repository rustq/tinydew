# MCP Task 05: Error Contract, Safety Controls, and Limits

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 5)

## Goal
Harden MCP behavior with strict error semantics, command allowlisting, and resource limits.

## Todo
- [x] Implement strict command/input validation.
- [x] Map errors to stable codes:
  - [x] `INVALID_COMMAND`
  - [x] `VALIDATION_ERROR`
  - [x] `SESSION_NOT_FOUND`
  - [x] `SESSION_CLOSED`
- [x] Standardize structured error object (`code`, `message`, optional `details`).
- [x] Enforce local-only MCP transport default.
- [x] Add runtime safeguards:
  - [x] max active sessions (default 10)
  - [x] idle timeout cleanup (default 30 min)
- [x] Ensure no arbitrary shell/system command execution path exists in MCP handlers.

## Acceptance
- [x] Invalid operations always return structured, predictable errors.
- [x] Session limits and idle cleanup behave as configured.
- [x] Security baseline is enforced by default.

## Notes
- Keep error wording human-readable, but clients must rely on codes, not free text.

---

## Completed Summary

**Implementation Details:**

1. **Error Codes** (`src/mcp/errors.rs`):
   - `INVALID_COMMAND` - returned for unknown commands
   - `VALIDATION_ERROR` - returned for invalid arguments/directions/crops
   - `SESSION_NOT_FOUND` - returned when session ID doesn't exist
   - `SESSION_CLOSED` - returned when accessing a closed session

2. **Structured Error Objects**:
   - `McpError` struct with `code`, `message`, and optional `details` fields
   - `ToolResponse` wraps all handler responses with `ok`, `result`, and `error` fields
   - Helper constructors: `McpError::invalid_command()`, `McpError::validation_error()`, etc.

3. **Input Validation** (`src/mcp/handler.rs`):
   - `validate_session_id()` function validates session IDs before processing
   - Checks for empty IDs and maximum length (256 chars)
   - Applied to all handlers: `handle_start_session`, `handle_end_session`, `handle_get_state`, `handle_get_map`, `handle_get_stats`, `handle_command`, `handle_command_batch`, `handle_resource_read`

4. **Command Allowlist** (`src/mcp/command.rs`):
   - `parse_command()` validates all commands against allowlist: move, clear, plant, water, harvest, buy, sell, sleep, print
   - Direction, crop type, and quantity validation with helpful error messages

5. **Runtime Safeguards** (`src/mcp/session.rs`):
   - `max_sessions = 10` - limits concurrent sessions
   - `idle_timeout_minutes = 30` - session cleanup threshold
   - `cleanup_idle_sessions()` method removes closed/expired sessions
   - `get_active_session_count()`, `get_max_sessions()`, `get_idle_timeout_minutes()` for monitoring
   - `create_session()` enforces max sessions limit with `VALIDATION_ERROR`

6. **Security**:
   - Local-only transport documented in `src/mcp/server.rs`
   - No shell/system command execution paths exist in MCP handlers
   - All game state modifications go through controlled command parsing

**Files Modified:**
- `src/mcp/session.rs` - Added idle cleanup and monitoring methods
- `src/mcp/handler.rs` - Added session_id validation to all handlers

**Tests:** 49 tests passing