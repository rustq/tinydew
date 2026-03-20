# Fishing Task 01: Baseline & Safety

Source:
- `agents/fishing.plan.md` (Phase 0)
- `agents/fishing.spec.md`

## Goal
Establish a safe baseline before introducing fishing behavior.

## Todo
- [x] Run baseline test suite and record status.
- [x] Verify current river tile behavior around SouthRiver/EastPath (walkability + rendering).
- [x] Identify and document current sleep/day reset hook used for daily world resets.
- [x] Capture any pre-existing issues that could affect fishing implementation.

## Acceptance
- [x] Baseline is green or known failures are documented.
- [x] River behavior + reset hook are confirmed and referenced for implementation tasks.
- [x] No unrelated behavior regressions introduced.

---

## Completed

### Baseline Test Suite
- **Status**: GREEN
- **Results**: 252 passed, 1 ignored, 0 failed
- **Ignored**: `test_mcp_command_batch_water_sleep_cycle` (sleep intentionally disabled in MCP API)

### SouthRiver Map Structure (11x4)
```
Row 0: 🌳🌳🌳🌳🌳🌿🌳🌳🌳🌳🌳  (PathSouthRiverGate at col 5)
Row 1: 🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳  (walkable grass)
Row 2: 🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊  (non-walkable river)
Row 3: 🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊  (non-walkable river)
```

### River Tile Behavior
- `TileType::River` (`🌊`): Non-walkable, blocks player/guest movement
- `TileType::RiverBubble` (`🫧`): Non-walkable, appears after fishing
- Walkability check: `src/world.rs:191-210` (`is_walkable()`)
- Rendering: `src/world.rs:245-246`

### EastPath Transition
- Entry to SouthRiver: `TileType::PathSouthRiver` at row 3, col 5
- Exit from SouthRiver: `TileType::PathSouthRiverGate` at row 0, col 5
- Transition handling: `src/state.rs:621-639`

### Sleep/Day Reset Hook
- **Main day reset**: `src/state.rs:656` (`start_new_day()`)
- **Bubble tile reset**: `src/state.rs:646-654` (`reset_bubble_tiles()`)
  - Converts `RiverBubble` → `River` for all SouthRiver tiles
- **Time advance**: `src/state.rs:979-1001` (`advance_minutes()`)
  - Triggers `start_new_day()` when `hour >= 24`
- **Auto-sleep check**: `src/state.rs:920-925` (`should_auto_sleep()`)
- **Auto-sleep execution**: `src/state.rs:927-930` (`run_auto_sleep()`)

### Pre-existing Fishing Support
- Fish inventory already exists: `src/state.rs:95-115`
- Fish types: Common (🐟, 80g), Rare (🐠, 180g) - `src/world.rs:83-107`
- Fishing action: `src/state.rs:1380-1441`
- Bubble tiles post-fishing: `src/state.rs:1435-1438`

## Notes
- No functional fishing changes in this task.
- Keep this task as a checkpoint for future debugging.
