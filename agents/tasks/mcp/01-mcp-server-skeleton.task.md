# MCP Task 01: Server Skeleton & Registration

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 1)

## Goal
Create the MCP server foundation for Shelldew and register the planned tools/resources.

## Todo
- [ ] Add MCP server module and startup wiring.
- [ ] Register tools (stub handlers acceptable initially):
  - [ ] `shelldew.start_session`
  - [ ] `shelldew.command`
  - [ ] `shelldew.command_batch`
  - [ ] `shelldew.get_state`
  - [ ] `shelldew.get_map`
  - [ ] `shelldew.get_stats`
  - [ ] `shelldew.end_session`
- [ ] Register resources:
  - [ ] `shelldew://session/{session_id}/state`
  - [ ] `shelldew://session/{session_id}/map`
  - [ ] `shelldew://session/{session_id}/inventory`
  - [ ] `shelldew://session/{session_id}/log/recent`
- [ ] Add basic request logging fields (tool/resource, session_id if present, duration).

## Acceptance
- [ ] MCP server starts successfully.
- [ ] MCP client can discover all tool names and resource URIs.
- [ ] Stub calls return structured placeholder responses instead of panics.

## Notes
- Keep transport local-only by default (security baseline).
- No gameplay logic required in this task beyond wiring.