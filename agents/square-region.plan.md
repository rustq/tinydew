# Square Region Implementation Plan

## Scope
Implement a new region: **Square**.

Square requirements from spec:
- New compact 9×5 Square region with center fountain, day-1 flower at (1,1), and boundary trees.
- Entry from East Path top opening tile (`🌿`) into Square.
- Exit from Square bottom-center opening tile (`🌿`) back to East Path.
- Fountain is a blocking building tile.
- **All Square tiles forbid `clear` and `plant`.**
- Preserve region-aware occupancy behavior for player/guest.
- Render + MCP + save/load compatibility.

---

## Phase 0 — Alignment & Baseline

### 0.1 Confirm exact transition pairing
- Define concrete transition mapping:
  - East Path top opening -> Square spawn tile
  - Square bottom opening -> East Path return tile
- Keep deterministic and symmetric enough for player intuition.

### 0.2 Baseline tests
- Run existing test suite sections touching:
  - movement/transition,
  - MCP print/map/state,
  - save/load.
- Capture baseline so regressions are obvious.

---

## Phase 1 — World Model & Tiles

### 1.1 Extend `Location`
- Add `Location::Square`.

### 1.2 Add Square map dimensions/constants
- Add `SQUARE_WIDTH = 9`, `SQUARE_HEIGHT = 5`.

### 1.3 Add building tile type
- Add `TileType::Fountain`.
- Rendering emoji: `⛲`.
- Walkability: non-walkable.

### 1.4 Add/confirm transition tile usage
- Reuse existing transition tile scheme or add new transition variant(s) to represent Square gateways.
- Ensure transition detection remains centralized and readable.

Deliverable:
- Code compiles with new enum/tile/constants in place.

---

## Phase 2 — State & Map Initialization

### 2.1 Add Square map to game state
- Add `square_map: Map` to `GameState`.
- Initialize map exactly as spec layout.

### 2.2 Update map accessors
- Update:
  - `get_current_map_ref()`
  - `get_current_map()`
  - `get_map_size()`
- Ensure `Location::Square` routes correctly.

### 2.3 Serialization compatibility
- Ensure save/load includes `Location::Square` and `square_map`.
- Confirm old saves still load safely (if applicable strategy exists).

Deliverable:
- Square map is fully represented in state and persisted.

---

## Phase 3 — Movement, Collision, and Transitions

### 3.1 Player movement constraints in Square
- Respect boundary/fountain non-walkable behavior.
- Preserve existing collision with guest only when same region.

### 3.2 Guest movement constraints in Square
- Same movement rules as player for walkability and region-aware collision.

### 3.3 Transition wiring
- Add East Path top opening transition into Square.
- Add Square bottom-center transition out to East Path.
- Ensure transitions update both:
  - active `location`
  - entity location field (`player_location`, `guest_location`)

### 3.4 Prevent accidental invalid transitions
- Ensure no other unintended Square transitions exist.

Deliverable:
- Player/guest can reliably move between East Path and Square.

---

## Phase 4 — Action Restrictions in Square

### 4.1 Global square policy for actions
- In Square:
  - `clear` always denied
  - `plant` always denied

### 4.2 Building action denial (fountain)
- Even outside global square deny path, targeting fountain must reject:
  - clear, plant, water, harvest

### 4.3 Message consistency
- Use consistent, user-readable rejection messages aligned with current style.

Deliverable:
- Action behavior strictly follows spec restrictions.

---

## Phase 5 — Rendering & MCP Surface

### 5.1 Interactive render updates
- Ensure `⛲` renders correctly.
- Ensure day-1 flower at (1,1) renders as `🌺`.
- Ensure map dimensions for Square render correctly.

### 5.2 MCP `print` snapshot updates
- Print correct 9×5 Square map.
- Ensure player/guest markers remain region-aware.

### 5.3 MCP `getMap` updates
- Return correct Square dimensions + tile encoding/details (including fountain and transition tiles).

### 5.4 MCP entity state separation updates
- Ensure `getState` snapshot includes guest block when guest is enabled.
- Ensure move command routes by `active_control`:
  - Guest active -> move guest only
  - Player active -> move player only
- Add regression coverage for independent player/guest position+location updates.

Deliverable:
- All user-facing map outputs correctly show Square.
- MCP state/control flow preserves independent player vs guest state.

---

## Phase 6 — Spawn/Growth Exclusions

### 6.1 Square spawn/growth policy
- No crop growth processing on Square.
- Allow day-based/random flower generation in Square.
- No random crop spawn on Square.
- No mushroom spawn on Square.

Deliverable:
- Square remains non-farm region over day progression.

---

## Phase 7 — Testing

## 7.1 Unit tests (minimum)
- `test_square_map_layout_dimensions_and_fountain_position`
- `test_square_fountain_not_walkable`
- `test_square_boundary_not_walkable`
- `test_player_cannot_move_onto_square_fountain`
- `test_guest_cannot_move_onto_square_fountain`
- `test_square_clear_forbidden_on_all_tiles`
- `test_square_plant_forbidden_on_all_tiles`
- `test_square_transition_enter_from_east_path`
- `test_square_transition_exit_to_east_path`
- `test_square_save_load_roundtrip`

### 7.2 MCP-focused tests
- `test_print_snapshot_renders_square_dimensions`
- `test_print_snapshot_contains_fountain_icon_in_square`
- `test_print_snapshot_contains_day1_flower_in_square`
- `test_get_map_returns_square_dimensions`
- `test_print_snapshot_hides_player_when_region_differs_in_square`

### 7.3 Regression checks
- Existing Farm/EastPath movement and actions unchanged.
- Existing guest region-aware occupancy behavior unchanged.

Deliverable:
- Tests pass and cover critical Square behavior.

---

## Phase 8 — Docs & Cleanup

### 8.1 Update docs
- Update feature/spec index docs (if present) to include Square.
- Add brief player-facing description of Square region.

### 8.2 Code cleanup
- Remove dead branches/temporary debug helpers.
- Ensure transition logic remains centralized and readable.

### 8.3 Final verification
- Manual MCP scenario:
  1. Move to East Path top opening.
  2. Enter Square.
  3. Try clear/plant on multiple Square tiles (should fail).
  4. Try interacting with fountain (blocked).
  5. Exit Square bottom gate back to East Path.
  6. Save + load + print to verify persistence.

---

## Risk Notes

1. **Map-size assumptions hardcoded elsewhere**
   - Mitigation: audit places assuming Farm/EastPath dimensions and switch to accessor-driven sizing.

2. **Transition ambiguity**
   - Mitigation: keep explicit transition mapping table in code comments.

3. **Action restrictions leaking through alternative action paths**
   - Mitigation: enforce policy in shared action gate instead of only command-level checks.

4. **Save compatibility drift**
   - Mitigation: add explicit save/load regression test for Square fields.

---

## Suggested Commit Sequence

1. `feat(world): add Square location, map constants, and fountain tile`
2. `feat(state): add square_map initialization and location routing`
3. `feat(transition): connect EastPath top entry with Square bottom gate`
4. `fix(actions): forbid clear/plant on all Square tiles`
5. `feat(render): support Square map and fountain in interactive/MCP outputs`
6. `test(square): add movement/action/transition/save-load coverage`
7. `docs(square): document Square region behavior and transition path`
