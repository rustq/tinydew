# MCP Task 05: Error Contract, Safety Controls, and Limits

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 5)

## Goal
Harden MCP behavior with strict error semantics, command allowlisting, and resource limits.

## Todo
- [ ] Implement strict command/input validation.
- [ ] Map errors to stable codes:
  - [ ] `INVALID_COMMAND`
  - [ ] `VALIDATION_ERROR`
  - [ ] `SESSION_NOT_FOUND`
  - [ ] `SESSION_CLOSED`
- [ ] Standardize structured error object (`code`, `message`, optional `details`).
- [ ] Enforce local-only MCP transport default.
- [ ] Add runtime safeguards:
  - [ ] max active sessions (default 10)
  - [ ] idle timeout cleanup (default 30 min)
- [ ] Ensure no arbitrary shell/system command execution path exists in MCP handlers.

## Acceptance
- [ ] Invalid operations always return structured, predictable errors.
- [ ] Session limits and idle cleanup behave as configured.
- [ ] Security baseline is enforced by default.

## Notes
- Keep error wording human-readable, but clients must rely on codes, not free text.