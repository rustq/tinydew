# Fishing Task 06: MCP/UI Output Integration

Source:
- `agents/fishing.plan.md` (Phase 5)
- `agents/fishing.spec.md` (UI / Rendering + MCP)

## Goal
Expose fishing and bubble state clearly in MCP outputs.

## Todo
- [x] Update snapshot/print inventory output to include fish counts.
- [x] Ensure map/tile serialization includes `RiverBubble` (`🫧`) in MCP outputs.
- [x] Ensure fishing result messages appear in bottom message area.
- [x] Ensure command responses include relevant result/event text for fishing attempts.

## Acceptance
- [x] MCP outputs accurately reflect fish inventory and bubble tile state.
- [x] User-facing fishing feedback is visible and clear.

---

## Completed

- Print/snapshot output already includes fish inventory lines in `src/mcp/command.rs`:
  - renders `Fish: <emoji> x<count>` entries in inventory section.
- MCP map/tile serialization already includes `RiverBubble` in both map encodings:
  - symbol mapping includes `RiverBubble -> "B"`
  - tile-name mapping includes `RiverBubble -> "RiverBubble"`
  - covered in `src/mcp/session.rs` and `src/mcp/state_manager.rs`.
- Fishing result messages are surfaced via `state.message` and displayed in snapshot bottom/message area.
- MCP command responses for fishing include:
  - `message` (catch/miss/error text)
  - event entry `"Fishing attempt"`.
- Verified with targeted tests:
  - `test_print_snapshot_contains_fish_inventory`
  - `test_execute_fishing_near_river`
  - `test_parse_fishing` / `test_parse_fishing_with_direction`
  - all passing.
