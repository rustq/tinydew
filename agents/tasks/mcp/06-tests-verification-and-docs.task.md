# MCP Task 06: Tests, Verification, and Usage Docs

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 6)

## Goal
Validate MCP contract stability and ship clear usage examples.

## Todo
- [ ] Add unit tests for:
  - [ ] session manager lifecycle
  - [ ] command validation
  - [ ] error mapping
  - [ ] schema serialization
- [ ] Add integration tests for:
  - [ ] start -> command(s) -> query -> end full lifecycle
  - [ ] batch stop-on-error behavior
  - [ ] resource read consistency with query tools
  - [ ] deterministic seed behavior
- [ ] Add docs/examples for MCP client usage flows.
- [ ] Run and pass verification gates:
  - [ ] `cargo fmt`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo test`

## Acceptance
- [ ] Tests cover major success and failure paths.
- [ ] All verification commands pass.
- [ ] Examples are sufficient for a developer to drive Shelldew via MCP.

## Definition of Done
- [ ] Tasks 01–05 complete
- [ ] Test suite green
- [ ] Docs updated with MCP usage guidance