# South River Region Spec

## Goal
Add a new explorable region named **South River**.

South River is a transition-connected map with:
- boundary trees (`🌳`) on top/side edges,
- walkable grass (`🌿`) in upper rows,
- a river band (`🌊`) across the lower area,
- one top-center entry/exit tile for transition routing.

---

## Canonical South River Layout

Map shape (11 columns × 4 rows):

```text
🌳🌳🌳🌳🌳🌿🌳🌳🌳🌳🌳
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊
🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊
```

### Notes
- Top opening is at row 0, col 5 (0-based indexing): this is the **South River gate tile**.
- All `🌳` tiles are boundaries and non-walkable.
- All `🌊` river tiles are non-walkable.
- Grass `🌿` is walkable.

---

## Transition Rules

### East Path -> South River
East Path must open a **middle-bottom** transition tile that leads into South River.

### South River -> East Path
South River top-center gate tile (`🌿` at row 0, col 5) transitions back to East Path.

### Transition Behavior
- Entering South River places player/guest at the South River top-center entry area.
- Exiting South River returns to East Path at the paired middle-bottom transition tile.
- Transition updates:
  - active `location`
  - entity location (`player_location` / `guest_location`)

---

## Movement & Collision

- Player and guest can move on grass tiles in South River.
- Player and guest cannot move onto:
  - tree boundary tiles,
  - river tiles,
  - occupied tile in the same region (existing region-aware occupancy rule).

River behavior:
- `🌊` blocks movement (like an obstacle band).

---

## Spawn Rules (Flowers)

South River supports random flower generation with these constraints:
- flowers can spawn only on **grass tiles** of South River,
- flowers cannot spawn on boundary tiles,
- flowers cannot spawn on river tiles,
- flowers cannot overwrite occupied/non-empty tiles.

South River is still non-farm:
- no random crop spawn in South River,
- no mushroom spawn in South River,
- no farming actions outside farm rules (existing behavior remains).

---

## Data Model Changes

### Location enum
Add:
- `SouthRiver`

### Map constants
Add:
- `SOUTH_RIVER_WIDTH = 11`
- `SOUTH_RIVER_HEIGHT = 4`

### Tile semantics
Reuse existing tiles where possible:
- `Boundary` for `🌳`
- `Grass` for `🌿`
- add/use a non-walkable `River` tile for `🌊` rendering + collision
- transition tiles for EastPath<->SouthRiver pairing

### GameState
Add:
- `south_river_map: Map`

Ensure save/load supports:
- `Location::SouthRiver`
- `south_river_map`

---

## Rendering Requirements

### Interactive / print rendering
- `get_current_map_ref()` / `get_map_size()` support `SouthRiver`.
- River tiles render as `🌊`.
- Top-center gate tile renders as `🌿`.
- Player/guest markers remain region-aware.

### MCP `getMap`
- Returns correct South River width/height and tile data.
- Includes river tile detail/code.

---

## Acceptance Criteria

1. **Map correctness**
   - South River layout matches canonical 11×4 map.

2. **Transitions**
   - East Path middle-bottom tile enters South River.
   - South River top-center tile exits to East Path.

3. **Collision**
   - River tiles block movement for player/guest.
   - Boundaries block movement.

4. **Spawns**
   - Random flowers can generate on South River grass only.
   - No random crop/mushroom generation in South River.

5. **Persistence**
   - Save/load preserves South River location/map state.

6. **Rendering/MCP**
   - South River renders correctly in interactive + MCP outputs.

---

## Suggested Tests

- `test_south_river_map_layout_dimensions`
- `test_south_river_river_tiles_not_walkable`
- `test_south_river_boundary_not_walkable`
- `test_transition_east_path_to_south_river`
- `test_transition_south_river_to_east_path`
- `test_guest_transition_east_path_to_south_river`
- `test_guest_transition_south_river_to_east_path`
- `test_south_river_random_flower_spawns_on_grass_only`
- `test_south_river_no_mushroom_spawn`
- `test_south_river_save_load_roundtrip`
- `test_print_snapshot_renders_south_river_dimensions`
- `test_get_map_returns_south_river_dimensions`
