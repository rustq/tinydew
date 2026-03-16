# Shelldew MCP Implementation Plan

Source:
- `agents/mcp.spec.md`

## Objective
Implement MCP support for Shelldew so MCP clients can create game sessions, execute gameplay commands, and query structured state through stable tool/resource contracts.

## Scope (Iteration 1)

Included:
- Local MCP server integration
- Stateful session lifecycle (`start_session` -> command/query -> `end_session`)
- Core tool set:
  - `shelldew.start_session`
  - `shelldew.command`
  - `shelldew.command_batch`
  - `shelldew.get_state`
  - `shelldew.get_map`
  - `shelldew.get_stats`
  - `shelldew.end_session`
- Read-only MCP resources for state/map/inventory/recent log
- Structured errors and deterministic execution support (`seed`)

Excluded:
- Multiplayer/shared sessions
- Remote auth providers
- Cloud save sync
- Replay export format

## Architecture Approach

1. **MCP transport layer**
   - Add MCP server entrypoint and tool/resource registration.
2. **Session manager**
   - Maintain in-memory map: `session_id -> GameState + metadata`.
3. **Command adapter**
   - Reuse existing Shelldew gameplay logic used by TUI/non-interactive mode.
4. **State serializers**
   - Convert internal state into stable MCP response schemas.
5. **Validation + error mapping**
   - Map parser/runtime failures into structured MCP error codes.

## Phased Plan

### Phase 1: MCP Server Skeleton
- Add MCP server module and startup wiring.
- Register all planned tool names and resource URIs.
- Add basic health logging and request tracing fields.

Deliverable:
- Server starts and advertises empty/stubbed tools/resources.

---

### Phase 2: Session Lifecycle
- Implement `shelldew.start_session`:
  - Create `session_id`
  - Initialize game state
  - Apply optional deterministic `seed`
- Implement `shelldew.end_session`:
  - Mark session closed
  - Release state/resources
- Implement session validation utilities:
  - `SESSION_NOT_FOUND`
  - `SESSION_CLOSED`

Deliverable:
- Stateful sessions can be created and closed safely.

---

### Phase 3: Command Tools
- Implement `shelldew.command` for single command execution.
- Implement command grammar and validation:
  - `move:*`, `clear`, `plant:<crop>`, `water`, `harvest`, `buy`, `sell`, `sleep`, `print`
- Implement `shelldew.command_batch`:
  - Ordered execution
  - `stop_on_error` behavior
- Return structured result payload:
  - `message`, `events`, optional `state_delta`, optional `snapshot_text`

Deliverable:
- MCP client can run full gameplay command loop via tools.

---

### Phase 4: Query Tools + Resources
- Implement query tools:
  - `shelldew.get_state`
  - `shelldew.get_map`
  - `shelldew.get_stats`
- Implement resources:
  - `shelldew://session/{session_id}/state`
  - `shelldew://session/{session_id}/map`
  - `shelldew://session/{session_id}/inventory`
  - `shelldew://session/{session_id}/log/recent`
- Ensure schema alignment between tools and resources.

Deliverable:
- Structured state retrieval available by both tool and resource access.

---

### Phase 5: Validation, Error Contract, and Safety Controls
- Enforce command allowlist and strict argument validation.
- Standardize error contract:
  - `INVALID_COMMAND`
  - `VALIDATION_ERROR`
  - `SESSION_NOT_FOUND`
  - `SESSION_CLOSED`
- Add safety controls:
  - Local-only transport default
  - Session limits (max active sessions)
  - Idle timeout cleanup

Deliverable:
- Predictable machine-readable failures and bounded runtime behavior.

---

### Phase 6: Testing & Verification
- Unit tests:
  - Session manager
  - Command validation
  - Error mapping
  - Serializer schemas
- Integration tests:
  - Full tool lifecycle flow
  - Batch stop-on-error behavior
  - Resource read consistency
  - Deterministic seed behavior
- Verification gates:
  - `cargo fmt`
  - `cargo clippy --all-targets --all-features`
  - `cargo test`

Deliverable:
- Green checks and confidence in MCP contract stability.

## Expected File Touch Points

- `Cargo.toml` (MCP dependencies)
- `src/main.rs` (optional server wiring/entrypoint)
- `src/mcp/mod.rs` (tool/resource registration)
- `src/mcp/session.rs` (session manager)
- `src/mcp/commands.rs` (command adapter)
- `src/mcp/resources.rs` (resource providers)
- `src/mcp/schema.rs` (response/error schemas)
- `src/mcp/errors.rs` (error codes/mapping)
- `tests/mcp_*.rs` (integration tests)

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Gameplay drift between MCP and TUI | Inconsistent behavior | Reuse existing action/state methods only |
| Unstable response payloads | Breaks automation clients | Freeze minimal schemas and test snapshots |
| Session leaks | Memory/resource growth | Idle timeout + explicit `end_session` + max sessions |
| Weak error semantics | Hard to automate retries | Enforce structured error codes and details |

## Rollout Checklist

- [ ] MCP server skeleton added and boots
- [ ] Session lifecycle tools implemented
- [ ] Single + batch command tools implemented
- [ ] Query tools and resources implemented
- [ ] Error contract fully mapped
- [ ] Safety limits enabled (local-only, timeout, max sessions)
- [ ] Unit/integration tests complete
- [ ] `fmt`, `clippy`, `test` all pass
- [ ] Basic usage examples documented

## Definition of Done

- MCP client can start a session, run end-to-end farming flow, query state/stats, and end session.
- Responses and errors are machine-readable and stable.
- All verification checks pass.
