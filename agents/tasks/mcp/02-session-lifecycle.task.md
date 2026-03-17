# MCP Task 02: Session Lifecycle Management

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 2)

## Goal
Implement reliable session lifecycle primitives for MCP gameplay interactions.

## Todo
- [x] Implement in-memory session manager (`session_id -> state + metadata`).
- [x] Implement `shelldew.start_session`:
  - [x] generate `session_id`
  - [x] initialize game state
  - [x] apply optional deterministic `seed`
  - [x] return `initial_state`
- [x] Implement `shelldew.end_session`:
  - [x] mark/close session
  - [x] release resources
- [x] Implement session validation helpers and error mapping:
  - [x] `SESSION_NOT_FOUND`
  - [x] `SESSION_CLOSED`
- [x] Add lifecycle metadata (created_at, last_activity_at) for timeout support.

## Acceptance
- [x] Sessions can be created and ended explicitly.
- [x] Closed sessions reject subsequent mutation/query calls with structured errors.
- [x] Unknown session IDs return `SESSION_NOT_FOUND`.

## Notes
- Keep this task focused on lifecycle; command execution comes next.

---

## Completed

Implemented complete session lifecycle management for MCP:

1. **Session Manager** (`src/mcp/session.rs`): In-memory session storage with thread-safe `SessionManager` supporting:
   - Session creation with optional seed and mode
   - Session lookup with closed state validation
   - Session closure with resource cleanup

2. **Error Handling** (`src/mcp/errors.rs`): Structured error codes including:
   - `SESSION_NOT_FOUND` - unknown session ID
   - `SESSION_CLOSED` - closed session access attempt
   - Added `PartialEq` for test comparisons
   - Added `NotImplemented` for future features

3. **Tool Handlers** (`src/mcp/handler.rs`): MCP tool implementation with:
   - `handle_start_session` - creates session, returns session_id + initial_state
   - `handle_end_session` - marks session as closed
   - `handle_get_state` - returns session state (validates session)
   - Lifecycle metadata: `created_at`, `last_accessed` timestamps
   - Comprehensive tests covering all acceptance criteria

4. **All acceptance criteria met**:
   - Sessions can be created and ended explicitly
   - Closed sessions reject calls with `SESSION_CLOSED` error
   - Unknown session IDs return `SESSION_NOT_FOUND`