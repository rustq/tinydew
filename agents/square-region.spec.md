# Square Region Spec

## Goal
Add a new explorable region named **Square**.

Square is a small public plaza map with:
- tree boundaries (`🌳`) as blocking edges,
- walkable grass interior (`🌿`),
- a center fountain (`⛲`) as a blocking building,
- a single bottom entry/exit tile used to enter/leave the region.

This is a functional map/region addition (navigation + collision + rendering + MCP/state compatibility), not a farming area.

---

## Canonical Square Layout

Map shape (11 columns × 6 rows):

```text
🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌳🌿🌿🌿🌿⛲🌿🌿🌿🌿🌳
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌳🌳🌳🌳🌳🌿🌳🌳🌳🌳🌳
```

### Notes
- Bottom opening is at row 5, col 5 (0-based indexing): this is the **Square gate tile**.
- All `🌳` tiles are boundaries and non-walkable.
- The fountain `⛲` at row 2, col 5 is a building and non-walkable.
- Grass `🌿` is walkable.

### East Path Entry Tile (for Square)
East Path should expose this top-row opening tile (`🌿`) so player/guest can enter Square from East Path:

```text
🌳🌳🌳🌳🌳🌿🌳🌳🌳🌳🌳
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳
🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳
```

---

## Gameplay Rules

### 1) Movement and Collision
- Player and guest can move on grass tiles in Square.
- Player and guest cannot move onto:
  - tree boundary tiles,
  - fountain tile,
  - occupied tile in the same region (existing region-aware collision rule).

### 2) Fountain Behavior
Fountain is a **building/obstacle**:
- cannot clear,
- cannot plant,
- cannot harvest,
- cannot water,
- cannot move onto.

When action targets the fountain tile, action should fail safely with normal invalid-target behavior/message style.

### 3) Farming/Foraging Restrictions
Square is non-farm gameplay space:
- **all Square tiles forbid `clear`**,
- **all Square tiles forbid `plant`**,
- no crop growth simulation in Square,
- no random crop spawn in Square,
- no mushroom spawn in Square.

### 4) Entry/Exit
- Square has one gate tile at bottom center.
- East Path must also expose an **entry tile `🌿` at top center** (0-based: `x = floor(EAST_PATH_WIDTH/2)`, `y = 0`).
- Stepping on that East Path top-center entry tile transitions player/guest into Square.
- Enter/leave behavior should use transition logic consistent with existing region transitions.
- Transition should update both:
  - active camera/view location (`location`),
  - entity region (`player_location`, `guest_location` as applicable).

---

## Data Model Changes

###[sic] Location enum
Add a new location variant:
- `Square`

### Map constants
Add new constants in world/map definitions:
- `SQUARE_WIDTH = 11`
- `SQUARE_HEIGHT = 6`

### Tile types
Add/introduce tile semantics for fountain and square transition:
- `Fountain` (non-walkable, building)
- transition tile(s) to connect Square to another region (implementation can use dedicated `PathSquare` / `PathFarm`-style tile or shared transition abstraction)

### GameState
Add square map storage:
- `square_map: Map` (shape 11×6)

Initialize Square map in `GameState::new()` with the exact canonical layout.

Ensure save/load serializes/deserializes Square map and `Location::Square`.

---

## Rendering Requirements

### Interactive render
- `get_current_map_ref()` / `get_map_size()` must support `Location::Square`.
- Fountain renders as `⛲`.
- Player/guest marker rendering remains region-aware:
  - only render marker when entity location matches active map location.

### MCP `print` snapshot
- Must correctly render Square map dimensions (11×6).
- Must display fountain icon at the center.
- Must not leak player marker to non-active region (same existing rule).

### MCP `getMap`
- Must return Square width/height and tile grid when in Square.
- Include correct tile codes/details for fountain and transition tile(s).

### MCP `getState` / snapshots (entity separation)
- State output must include both independent entities when guest is enabled:
  - `player`: `x`, `y`, `location`
  - `guest`: `x`, `y`, `location`, `enabled`, `active`
- `active_control=Guest` movement must update guest coordinates/location only.
- `active_control=Player` movement must update player coordinates/location only.

---

## Command/Action Semantics

Existing commands should remain valid globally, with region-aware behavior:
- `move:*` works in Square within collision constraints.
- `clear[:dir]`, `plant:<crop>[:dir]`, `water[:dir]`, `harvest[:dir]` on fountain tile must be rejected as invalid target.
- `print`, `save`, `load` behave unchanged.

No new MCP commands are required for this feature.

---

## Transition Mapping (Implementation Contract)

Implement a deterministic mapping between adjacent regions and Square gate tile.

Minimum requirement:
- East Path top-center entry tile (`🌿`) transitions into Square.
- A player can leave Square via the bottom-center gate and return to East Path in a logical paired position.
- (Optional extension) Additional entries from other regions may be added later.

Guest transitions should follow the same path rules when guest movement enters the transition tile.

---

## Non-Goals

- No shop/NPC/dialog system in Square.
- No special fountain interaction mechanics beyond obstacle/building behavior.
- No weather-specific Square events.

---

## Acceptance Criteria

1. **Map correctness**
   - Square map is exactly 11×6 with center fountain and bottom-center opening.

2. **Collision correctness**
   - Player cannot move onto tree/fountain.
   - Guest cannot move onto tree/fountain.
   - Occupancy checks remain region-aware.

3. **Action restrictions**
   - `clear` is not allowed on any Square tile.
   - `plant` is not allowed on any Square tile.
   - water/harvest/clear/plant cannot affect fountain.

4. **Transitions**
   - Player can enter and leave Square through transition tiles.
   - Guest can also transition when applicable.

5. **Rendering**
   - Interactive and MCP print render Square correctly (size + icons).
   - Player/guest markers shown only in active matching region.

6. **Persistence**
   - Save/load preserves Square location, map state, and entity region data.

7. **Regression safety**
   - Existing Farm/EastPath behavior remains unchanged.

---

## Suggested Tests

### Unit tests
- `test_square_map_layout_dimensions_and_fountain_position`
- `test_square_fountain_not_walkable`
- `test_square_boundary_not_walkable`
- `test_player_cannot_move_onto_square_fountain`
- `test_guest_cannot_move_onto_square_fountain`
- `test_square_actions_reject_fountain_target`
- `test_square_plant_blocked_on_all_square_tiles`
- `test_square_transition_enter_and_exit_player`
- `test_square_transition_enter_and_exit_guest`
- `test_square_save_load_roundtrip`

### MCP snapshot tests
- `test_print_snapshot_renders_square_dimensions`
- `test_print_snapshot_contains_fountain_icon_in_square`
- `test_print_snapshot_hides_player_when_player_region_differs_in_square`

---

## Implementation Notes (Optional Guidance)

- Prefer introducing explicit `TileType::Fountain` to avoid overloading existing tile semantics.
- Keep transition handling centralized (e.g., in `handle_transition`) to reduce map-specific branching spread.
- Reuse existing region-aware occupancy checks for player/guest.
- Ensure any map-size assumptions in print/snapshot code use `get_map_size()` rather than hardcoded farm/eastpath dimensions.
