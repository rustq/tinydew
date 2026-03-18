# Guest Girl Interactive Mode — Implementation Plan

## References

- Spec: `agents/guest-girl-interactive-mode.spec.md`
- Existing runtime notes: `agents/non-interactive-mode.spec.md`, `agents/mcp.spec.md`

---

## Objective

Implement an interactive-only guest girl (`👧`) character that can be controlled with arrow keys, move across Farm/East Path, appear in UI (including MCP snapshots), but cannot perform any non-movement actions.

---

## Delivery Strategy

Build in focused phases with low regression risk:

1. Add guest data model + control target in core state.
2. Implement movement/collision/transition rules for guest.
3. Reintroduce/solidify interactive TUI control loop with control toggle.
4. Update rendering for both entities and active-control status.
5. Ensure save/load and MCP snapshot visibility compatibility.
6. Add tests and docs.

---

## Phase 0 — Baseline and Safety Checks

### Tasks

1. Confirm current runtime split:
   - MCP command runtime still functioning
   - interactive mode path availability/entrypoint status
2. Add baseline tests for current map rendering and movement behavior.
3. Identify all direct assumptions that only one controllable character exists.

### Files likely touched

- `src/main.rs`
- `src/state.rs`
- `src/mcp/command.rs`

### Exit criteria

- Baseline tests pass and current behavior is captured before guest changes.

---

## Phase 1 — Data Model for Guest + Control Target

### Tasks

1. Add state fields:
   - `guest_enabled: bool`
   - `guest_x: usize`
   - `guest_y: usize`
   - `guest_location: Location`
   - `active_control: ControlTarget`
2. Add enum:
   - `ControlTarget::{Player, Guest}`
3. Initialize defaults:
   - interactive mode start => guest enabled with valid spawn
   - non-interactive/MCP startup => guest disabled
4. Add helper methods:
   - `enable_guest_for_interactive()`
   - `disable_guest()`
   - `is_guest_on_current_map()`

### Files likely touched

- `src/state.rs`
- `src/savegame.rs` (if explicit migration helpers needed)

### Exit criteria

- State compiles with guest/control fields.
- Guest lifecycle can be toggled cleanly.

---

## Phase 2 — Guest Spawn + Collision Rules

### Tasks

1. Implement guest spawn on Farm walkable tile.
2. Add fallback search for nearest walkable tile if default is blocked.
3. Add separation rule:
   - guest cannot enter player tile
   - player cannot enter guest tile when guest enabled on same map
4. Standardize collision message:
   - `Tile occupied.`

### Files likely touched

- `src/state.rs`

### Exit criteria

- Guest always spawns validly.
- Player/guest overlap is prevented.

---

## Phase 3 — Guest Movement + Map Transition

### Tasks

1. Add guest movement method(s), e.g.:
   - `move_guest(Direction)`
2. Reuse map walkability checks for guest.
3. Implement guest transitions Farm↔EastPath at path connectors.
4. Keep player transition logic unchanged.

### Files likely touched

- `src/state.rs`
- `src/world.rs` (only if shared helpers required)

### Exit criteria

- Guest can walk around both maps and cross connectors.
- Movement blocks correctly at boundaries/obstacles/player position.

---

## Phase 4 — Interactive Control Toggle

### Tasks

1. Add control toggle key in interactive mode:
   - `Tab` switches `active_control` between Player/Guest
2. Arrow keys move whichever control target is active.
3. Non-movement action keys behavior:
   - if Player active => existing behavior
   - if Guest active => blocked with `Guest can only walk around.`
4. Keep existing exit/back controls stable.

### Files likely touched

- `src/main.rs`
- `src/state.rs`

### Exit criteria

- Interactive mode fully supports Player/Guest control switching.

---

## Phase 5 — Rendering & UX Updates

### Tasks

1. Render guest emoji `👧` on map when on current location.
2. Preserve deterministic render priority if overlap edge-case occurs:
   - active-controlled entity wins visual priority
3. Update footer/help text:
   - Arrow keys move active entity
   - Tab switches control target
4. Show control indicator:
   - `Control: Player` or `Control: Guest`

### Files likely touched

- `src/main.rs`
- `src/mcp/command.rs` (text snapshot rendering)

### Exit criteria

- UI clearly shows both entities and current control mode.

---

## Phase 6 — MCP Snapshot Compatibility (Visibility-Only)

### Tasks

1. Include guest in snapshot outputs when guest is enabled:
   - text snapshot map rendering should show `👧`
   - structured state/map may include guest fields for observability
2. Explicitly block MCP control of guest:
   - no guest-specific movement command added
   - existing MCP command grammar unchanged

### Files likely touched

- `src/mcp/command.rs`
- `src/mcp/handler.rs` (if state schema extended)
- `agents/mcp.spec.md` (doc update)

### Exit criteria

- MCP can see guest state but cannot control guest.

---

## Phase 7 — Save/Load + Interactive Lifecycle

### Tasks

1. Persist guest fields in save/load schema.
2. Apply lifecycle rule from spec:
   - interactive run enables guest
   - exiting interactive mode disables guest
3. On load where guest invalid tile is detected:
   - relocate to nearest valid walkable tile

### Files likely touched

- `src/savegame.rs`
- `src/state.rs`

### Exit criteria

- Save/load works without corrupting existing saves.
- Guest only appears during interactive runtime as required.

---

## Phase 8 — Tests and Verification

### Required unit tests

1. Guest spawn validity + fallback
2. Guest movement boundaries/walkability
3. Guest transition Farm↔EastPath
4. Player-guest collision blocking
5. Control toggle behavior
6. Guest non-movement action blocking
7. Time remains frozen while guest interactive mode is active
8. Save/load roundtrip with guest fields
9. MCP snapshot contains guest when enabled (visibility only)

### Required integration/manual checks

1. Start interactive mode, confirm guest appears.
2. `Tab` to guest, move around both maps.
3. Attempt non-movement key while guest active -> blocked message.
4. Exit interactive mode, restart non-interactive/MCP mode -> guest absent.
5. Re-enter interactive mode -> guest enabled again.

### Exit criteria

- Tests pass and manual checks match spec decisions.

---

## Suggested Commit Breakdown

1. **commit A**: state model (guest fields + control target enum)
2. **commit B**: guest movement/collision/transition logic
3. **commit C**: interactive key handling + control toggle
4. **commit D**: rendering updates + footer/help control text
5. **commit E**: save/load lifecycle behavior for interactive-only guest
6. **commit F**: MCP snapshot visibility + docs sync
7. **commit G**: tests + cleanup

---

## Risk Notes & Mitigations

### Risk 1: Reintroducing interactive mode conflicts with MCP runtime
- **Mitigation:** keep explicit runtime mode switch; do not alter MCP command flow.

### Risk 2: Save compatibility breaks older save files
- **Mitigation:** default missing guest fields on deserialize; add migration-safe defaults.

### Risk 3: Entity overlap bugs
- **Mitigation:** centralize occupancy checks and enforce both-direction collision guards.

### Risk 4: Confusing controls for users
- **Mitigation:** persistent `Control: ...` label + clear key hints in footer.

---

## Definition of Done

- Guest girl (`👧`) is interactive-mode only and controllable by arrow keys.
- `Tab` toggles Player/Guest control.
- Guest can walk in Farm/EastPath and transition between them.
- Guest cannot perform non-movement actions.
- Player blocks guest movement (no overlap).
- Guest visible in MCP snapshots but MCP cannot control guest.
- Save/load and regression tests pass.
 actions.
- Player blocks guest movement (no overlap).
- Guest visible in MCP snapshots but MCP cannot control guest.
- Save/load and regression tests pass.
