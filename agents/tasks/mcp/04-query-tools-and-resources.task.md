# MCP Task 04: Query Tools & Read-Only Resources

Source:
- `agents/mcp.spec.md`
- `agents/mcp.plan.md` (Phase 4)

## Goal
Expose stable read APIs for state, map, inventory, and stats via both tools and resources.

## Todo
- [x] Implement query tools:
  - [x] `shelldew.get_state` (already implemented in Task 02)
  - [x] `shelldew.get_map` (implemented in handler.rs)
  - [x] `shelldew.get_stats` (implemented in handler.rs)
- [x] Ensure minimum `get_state/get_stats` fields:
  - [x] `day`
  - [x] `time`
  - [x] `location`
  - [x] `money`
  - [x] `inventory`
  - [x] `status`
  - [x] `player` (`get_state`)
- [x] Implement read-only resources:
  - [x] `.../state`
  - [x] `.../map`
  - [x] `.../inventory`
  - [x] `.../log/recent`
- [x] Align resource payload schemas with tool payload schemas.

## Acceptance
- [x] Tool-based queries return expected structured payloads.
- [x] Resource reads succeed for active sessions.
- [x] Tools/resources stay schema-consistent for equivalent data.

## Notes
- `get_map` returns a 2D array of tile codes (stable format documented in session.rs):
  - "X": Boundary
  - ".": Grass
  - "#": Soil
  - "H": House
  - "P": PathEast/PathFarm
  - "M": Mushroom
  - "C": Crop (generic)
  - When include_entities is true, crops show detailed info (type, growth days, mature/growing)

## Implementation Details
- Query tools implemented in `src/mcp/handler.rs`:
  - `handle_get_state()` - returns state snapshot (already existed)
  - `handle_get_map()` - returns map with tiles, location, player position
  - `handle_get_stats()` - returns stats with day, time, location, money, inventory, status, season, weather
- Resources implemented via `handle_resource_read()` in `src/mcp/handler.rs`
- Resource URI parsing in `src/mcp/resources.rs::McpResources::parse_resource_uri()`
- Session helper methods in `src/mcp/session.rs`:
  - `to_snapshot()` - for state resource
  - `to_map_snapshot(include_entities)` - for map resource
  - `to_inventory_snapshot()` - for inventory resource
  - `to_log_snapshot(limit)` - for log/recent resource
  - `to_stats()` - for get_stats tool
- Log tracking added to Session with `Vec<LogEntry>`

## Completed: 2026-03-17