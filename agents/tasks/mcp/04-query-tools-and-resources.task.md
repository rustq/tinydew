# MCP Task 04: Query Tools & Read-Only Resources

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 4)

## Goal
Expose stable read APIs for state, map, inventory, and stats via both tools and resources.

## Todo
- [ ] Implement query tools:
  - [ ] `shelldew.get_state`
  - [ ] `shelldew.get_map`
  - [ ] `shelldew.get_stats`
- [ ] Ensure minimum `get_state/get_stats` fields:
  - [ ] `day`
  - [ ] `time`
  - [ ] `location`
  - [ ] `money`
  - [ ] `inventory`
  - [ ] `status`
  - [ ] `player` (`get_state`)
- [ ] Implement read-only resources:
  - [ ] `.../state`
  - [ ] `.../map`
  - [ ] `.../inventory`
  - [ ] `.../log/recent`
- [ ] Align resource payload schemas with tool payload schemas.

## Acceptance
- [ ] Tool-based queries return expected structured payloads.
- [ ] Resource reads succeed for active sessions.
- [ ] Tools/resources stay schema-consistent for equivalent data.

## Notes
- `get_map` may return 2D arrays or row strings, but format must be documented and stable.