# Fishing Task 02: Data Model Extensions

Source:
- `agents/fishing.plan.md` (Phase 1)
- `agents/fishing.spec.md` (Data Model Changes)

## Goal
Add core model types for fish items and river bubble tile state.

## Todo
- [x] Add `FishType` enum with:
  - [x] `FishCommon` (`🐟`, value `$80`)
  - [x] `FishRare` (`🐠`, value `$180`)
- [x] Extend inventory model with fish storage (e.g., `fish: HashMap<FishType, u32>`).
- [x] Add inventory helpers:
  - [x] `add_fish(...)`
  - [x] `sell_fish(...)`
  - [x] optional `fish_count(...)`
- [x] Add `TileType::RiverBubble` (`🫧`) to tile model.
- [x] Ensure `RiverBubble` is:
  - [x] non-walkable
  - [x] fishable

## Acceptance
- [x] Project compiles with fish + bubble model additions.
- [x] Existing gameplay paths unaffected by default.

---

## Completed

- `FishType` already exists in `src/world.rs` with:
  - `Common` (`🐟`) -> `price() == 80`
  - `Rare` (`🐠`) -> `price() == 180`
- Inventory already includes fish storage and helpers in `src/state.rs`:
  - `fish: HashMap<FishType, u32>`
  - `add_fish(...)`
  - `sell_fish(...)`
  - `fish_count(...)`
- `TileType::RiverBubble` (`🫧`) already exists in `src/world.rs`.
- `RiverBubble` behavior already matches requirements:
  - non-walkable via `TileType::is_walkable()`
  - fishable via fishing target checks in `state.rs`
- Existing tests already cover fish and bubble model behavior; no additional model code changes were required for Task 02.

## Notes
- Keep behavioral logic (time/probability/target checks) for later tasks.
