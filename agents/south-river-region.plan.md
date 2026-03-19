# South River Region Implementation Plan

## Scope
Implement a new region: **South River**.

Requirements from spec:
- Add South River map (11×4) with top-center gate, grass area, and blocking river rows.
- Add East Path middle-bottom transition into South River.
- Add South River top-center transition back to East Path.
- River (`🌊`) is non-walkable.
- Random flower generation is allowed on South River grass tiles.
- Random crop/mushroom generation must not happen in South River.
- Support player+guest transitions, MCP rendering, and save/load.

---

## Phase 0 — Baseline

1. Run current tests as baseline.
2. Confirm East Path transition coordinates to pair with South River gate.

---

## Phase 1 — World Model

1. Add `Location::SouthRiver`.
2. Add constants:
   - `SOUTH_RIVER_WIDTH = 11`
   - `SOUTH_RIVER_HEIGHT = 4`
3. Add river tile type (`River`) with emoji `🌊` and non-walkable behavior.
4. Add `create_south_river_map()` with exact canonical layout.

Deliverable:
- World compiles with South River map + river tile.

---

## Phase 2 — State + Persistence

1. Add `south_river_map: Map` to `GameState`.
2. Initialize South River map in `GameState::new()`.
3. Update map accessors:
   - `get_current_map_ref()`
   - `get_current_map()`
   - `get_map_size()`
4. Ensure save/load includes South River state and location enum value.

Deliverable:
- South River can be current location and persists through save/load.

---

## Phase 3 — Transition Wiring

1. Add East Path middle-bottom transition tile into South River.
2. Add South River top-center gate transition back to East Path.
3. Update player transition handling.
4. Update guest transition handling.

Deliverable:
- Player and guest can round-trip EastPath <-> SouthRiver.

---

## Phase 4 — Movement + Collision

1. Ensure river tiles block movement.
2. Ensure boundary tiles block movement.
3. Keep region-aware occupancy logic unchanged.

Deliverable:
- No movement onto river or boundaries.

---

## Phase 5 — Spawn Rules

1. Extend random flower generation to include South River grass tiles.
2. Ensure flower spawn never lands on:
   - river,
   - boundary,
   - occupied/non-empty tiles.
3. Keep random crop spawn excluded from South River.
4. Keep mushroom spawn excluded from South River.

Deliverable:
- South River gets flowers on valid grass tiles only.

---

## Phase 6 — Rendering + MCP

1. Render South River map and river emoji in print/interactive paths.
2. Ensure MCP `print` supports South River dimensions.
3. Ensure MCP `getMap` includes South River width/height and tile details.

Deliverable:
- South River visible and correct in UI + MCP outputs.

---

## Phase 7 — Tests

Add/adjust tests:
- `test_south_river_map_layout_dimensions`
- `test_south_river_river_tiles_not_walkable`
- `test_transition_east_path_to_south_river`
- `test_transition_south_river_to_east_path`
- `test_guest_transition_east_path_to_south_river`
- `test_guest_transition_south_river_to_east_path`
- `test_south_river_random_flower_spawns_on_grass_only`
- `test_south_river_no_mushroom_spawn`
- `test_south_river_save_load_roundtrip`
- `test_print_snapshot_renders_south_river_dimensions`
- `test_get_map_returns_south_river_dimensions`

Deliverable:
- Green test suite with South River coverage.

---

## Phase 8 — Final Verification

Manual MCP flow:
1. Start from Farm.
2. Move to East Path.
3. Enter South River via East Path middle-bottom transition.
4. Verify river blocks movement.
5. Exit South River via top-center gate back to East Path.
6. Save/load and re-check position + map.

---

## Suggested Commit Sequence

1. `feat(world): add SouthRiver location, map, and river tile`
2. `feat(state): add south_river_map and persistence support`
3. `feat(transition): connect EastPath and SouthRiver gates`
4. `feat(spawn): allow random flowers on SouthRiver grass`
5. `feat(mcp): render and expose SouthRiver map/state`
6. `test(south-river): add transitions/collision/spawn/save coverage`
7. `docs(south-river): finalize spec and plan alignment`
