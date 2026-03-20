# Fishing Task 07: Persistence

Source:
- `agents/fishing.plan.md` (Phase 6)
- `agents/fishing.spec.md` (Acceptance: save/load)

## Goal
Persist fishing-related state across save/load.

## Todo
- [x] Include fish inventory in save format.
- [x] Include `RiverBubble` tile states in save format.
- [x] Verify load path restores fish counts and bubble tiles correctly.
- [x] Handle backward compatibility for old saves where possible.

## Acceptance
- [x] Save/load roundtrip preserves fish and bubble state.
- [x] Legacy save behavior is documented if migrations/defaulting are required.

---

## Completed

- Save format persists full `GameState` via serde (`SaveGameData { version, state }` in `src/savegame.rs`).
- `GameState` includes fish inventory and SouthRiver map tiles (including `RiverBubble`) through derived serialization.
- Verified fish inventory persistence with existing test:
  - `test_fish_inventory_persists_save_load`
- Verified bubble tile persistence with existing test:
  - `test_river_bubble_state_persists_save_load`
- Backward compatibility status:
  - Save format includes `version: 1` metadata, but no explicit migration layer for older schema variants is currently implemented.
  - Current behavior expects compatible JSON shape for `GameState` deserialization.
