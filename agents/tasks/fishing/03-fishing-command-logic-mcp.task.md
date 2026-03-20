# Fishing Task 03: Fishing Command Logic (MCP)

Source:
- `agents/fishing.plan.md` (Phase 2)
- `agents/fishing.spec.md` (Action Rules + MCP)

## Goal
Implement end-to-end MCP `fishing` action behavior.

## Todo
- [x] Add MCP command parser support for `fishing`.
- [x] (Optional) Add directional variant `fishing:<dir>` if consistent with command grammar.
- [x] Enforce valid target rules:
  - [x] fishing only when near/targeting `River` or `RiverBubble`
  - [x] invalid attempt returns clear message (e.g., `No river nearby to fish.`)
  - [x] invalid attempt does **not** advance time
- [x] Implement outcome probabilities exactly:
  - [x] 20% -> `FishCommon` (`🐟`)
  - [x] 70% -> nothing
  - [x] 10% -> `FishRare` (`🐠`)
- [x] On fish success:
  - [x] add fish to inventory
  - [x] show congratulatory bottom message
- [x] On nothing:
  - [x] show neutral bottom message
- [x] Convert targeted `River` tile to `RiverBubble` after valid attempt.
- [x] Keep `RiverBubble` as `RiverBubble` if targeted again.
- [x] Advance time by 60 minutes on valid fishing attempt.

## Acceptance
- [x] `fishing` works through MCP command flow.
- [x] Time, inventory, tile mutation, and messaging all match spec.

---

## Completed

- MCP parser supports:
  - `fishing`
  - `fishing:<dir>` (up/down/left/right)
  via `src/mcp/command.rs`.
- Execution path wired through MCP command executor into state fishing actions:
  - `state.fishing_action()`
  - `state.fishing_action_at(dir)`
- Target validity and failure behavior implemented in `src/state.rs`:
  - only `River`/`RiverBubble` are valid targets
  - invalid attempts set `"No river nearby to fish."`
  - invalid attempts do not advance time
- Fishing outcomes implemented with exact probabilities:
  - 20% common fish (`🐟`)
  - 70% nothing
  - 10% rare fish (`🐠`)
- Success/failure messaging implemented:
  - success: congratulatory messages for `🐟` / `🐠`
  - miss: `"No bite this time."`
- Tile mutation implemented:
  - targeted `River` -> `RiverBubble`
  - targeted `RiverBubble` remains bubble
- Time cost implemented:
  - valid fishing attempt calls `advance_minutes(60)`.
- Existing tests already cover parser + core behavior:
  - `test_parse_fishing`
  - `test_parse_fishing_with_direction`
  - `test_execute_fishing_near_river`
  - `test_execute_fishing_no_river_nearby`
  - `test_execute_fishing_advances_time`
  - plus state-level outcome/tile/message tests.
- Verified current baseline after documentation update:
  - `cargo test`: 252 passed, 1 ignored, 0 failed.

## Notes
- Reuse existing RNG/deterministic framework used by other actions.
