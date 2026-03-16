# MCP Task 02: Session Lifecycle Management

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 2)

## Goal
Implement reliable session lifecycle primitives for MCP gameplay interactions.

## Todo
- [ ] Implement in-memory session manager (`session_id -> state + metadata`).
- [ ] Implement `shelldew.start_session`:
  - [ ] generate `session_id`
  - [ ] initialize game state
  - [ ] apply optional deterministic `seed`
  - [ ] return `initial_state`
- [ ] Implement `shelldew.end_session`:
  - [ ] mark/close session
  - [ ] release resources
- [ ] Implement session validation helpers and error mapping:
  - [ ] `SESSION_NOT_FOUND`
  - [ ] `SESSION_CLOSED`
- [ ] Add lifecycle metadata (created_at, last_activity_at) for timeout support.

## Acceptance
- [ ] Sessions can be created and ended explicitly.
- [ ] Closed sessions reject subsequent mutation/query calls with structured errors.
- [ ] Unknown session IDs return `SESSION_NOT_FOUND`.

## Notes
- Keep this task focused on lifecycle; command execution comes next.