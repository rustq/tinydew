# MCP Task 01: Server Skeleton & Registration

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 1)

## Goal
Create the MCP server foundation for Shelldew and register the planned tools/resources.

## Todo
- [x] Add MCP server module and startup wiring.
- [x] Register tools (stub handlers acceptable initially):
  - [x] `shelldew.start_session`
  - [x] `shelldew.command`
  - [x] `shelldew.command_batch`
  - [x] `shelldew.get_state`
  - [x] `shelldew.get_map`
  - [x] `shelldew.get_stats`
  - [x] `shelldew.end_session`
- [x] Register resources:
  - [x] `shelldew://session/{session_id}/state`
  - [x] `shelldew://session/{session_id}/map`
  - [x] `shelldew://session/{session_id}/inventory`
  - [x] `shelldew://session/{session_id}/log/recent`
- [x] Add basic request logging fields (tool/resource, session_id if present, duration).

## Acceptance
- [x] MCP server starts successfully.
- [x] MCP client can discover all tool names and resource URIs.
- [x] Stub calls return structured placeholder responses instead of panics.

## Notes
- Keep transport local-only by default (security baseline).
- No gameplay logic required in this task beyond wiring.

## Completed
- MCP foundation implemented and wired in app entrypoints.
- Tool and resource registration added per spec-defined names/URIs.
- Structured stub responses returned by handlers to avoid panic paths.
- Request-level logging/tracing fields added for MCP calls.
- Initial implementation merged via commit `fe803b3`.
