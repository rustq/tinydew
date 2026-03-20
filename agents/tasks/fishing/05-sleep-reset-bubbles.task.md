# Fishing Task 05: Sleep Reset for Bubble Tiles

Source:
- `agents/fishing.plan.md` (Phase 4)
- `agents/fishing.spec.md` (Action Rule 2.1)

## Goal
Reset all river bubbles back to river on sleep/day transition.

## Todo
- [x] Hook into sleep-cycle/day-reset path.
- [x] Iterate applicable map regions containing river tiles.
- [x] Replace all `RiverBubble` -> `River` during reset.
- [x] Ensure reset runs exactly once per sleep completion/day transition.

## Acceptance
- [x] Bubble reset is deterministic and complete after sleep cycle.
- [x] No unintended mutation of non-river tiles.

---

## Completed

- Sleep/day reset hook already exists and is wired:
  - `start_new_day()` calls `reset_bubble_tiles()` in `src/state.rs`.
- Reset logic is implemented for the river region map:
  - iterates the full SouthRiver grid
  - converts each `TileType::RiverBubble` to `TileType::River`
  - leaves non-bubble tiles unchanged.
- Reset execution cadence is tied to day transition:
  - day transition runs through `start_new_day()`
  - bubble reset occurs once per day-reset invocation.
- Deterministic behavior verified with targeted tests:
  - `test_sleep_cycle_resets_bubble_to_river`
  - `test_river_tile_turns_into_bubble_after_fishing`
  - `test_river_bubble_tile_is_still_fishable`
  - all passing.
