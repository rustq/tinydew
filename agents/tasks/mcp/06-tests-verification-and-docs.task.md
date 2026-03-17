# MCP Task 06: Tests, Verification, and Usage Docs

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 6)

## Goal
Validate MCP contract stability and ship clear usage examples.

## Todo
- [x] Add unit tests for:
  - [x] session manager lifecycle
  - [x] command validation
  - [x] error mapping
  - [x] schema serialization
- [x] Add integration tests for:
  - [x] start -> command(s) -> query -> end full lifecycle
  - [x] batch stop-on-error behavior
  - [x] resource read consistency with query tools
  - [x] deterministic seed behavior
- [x] Add docs/examples for MCP client usage flows.
- [ ] Run and pass verification gates:
  - [ ] `cargo fmt`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo test`

## Acceptance
- [x] Tests cover major success and failure paths.
- [ ] All verification commands pass.
- [x] Examples are sufficient for a developer to drive Shelldew via MCP.

## Definition of Done
- [x] Tasks 01–05 complete
- [ ] Test suite green
- [x] Docs updated with MCP usage guidance

---

## Completed Summary

### Unit Tests Added
- **Session Manager Lifecycle** (`src/mcp/session.rs`): 14 new tests covering session creation, retrieval, closing, cleanup, and limits
- **Command Validation** (`src/mcp/command.rs`): 18 new tests covering edge cases (empty/whitespace commands, missing args, case insensitivity, all crop types)
- **Error Mapping** (`src/mcp/errors.rs`): 12 new tests covering error code serialization, error construction, and message formatting
- **Schema Serialization** (`src/mcp/schema.rs`): 7 new tests covering serialization/deserialization and roundtrip

### Integration Tests Added
- **Full MCP Lifecycle** (`src/mcp/handler.rs`): Tests complete flow: start -> commands -> state/map/stats queries -> end
- **Batch Stop-on-Error**: Tests both stop_on_error=true and stop_on_error=false behavior
- **Resource Read Consistency**: Tests that resource reads return same data as query tools
- **Deterministic Seed**: Tests that same seed produces identical game states

### Documentation Added
- Updated `agents/tasks/mcp/README.md` with comprehensive MCP usage guide including:
  - Available tools and resources
  - Quick start examples
  - Command reference table
  - Batch command usage
  - Error handling guide
  - Deterministic seeding
  - Resource access examples
  - Session limits