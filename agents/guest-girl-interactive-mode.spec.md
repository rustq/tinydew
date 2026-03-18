# Guest Girl Interactive Mode — Spec

## 1) Objective

Add a controllable **guest character** (girl emoji) that can enter the game world and walk around.

Core intent:
- Guest is visible on map as a girl emoji.
- Guest can move in Farm and East Path.
- Guest cannot perform any gameplay actions (no clear/plant/water/harvest/buy/sell/sleep).
- Guest is controlled in **interactive TUI mode** with arrow keys.

---

## 2) Feature Summary

### New capability
A second controllable character exists in interactive mode:
- **Player** (existing character)
- **Guest girl** (new character, emoji-based)

### Control model
In interactive mode, keyboard arrows can control the guest girl (control toggle defined below).

### World scope
Guest can move across:
- Farm
- East Path

Guest follows map collision rules for walking but has no interaction verbs.

---

## 3) Scope

### In scope
1. Guest entity data model (position, location, active state)
2. Guest rendering in TUI map
3. Guest movement using arrow keys in interactive mode
4. Guest map transitions between Farm/East Path
5. Control switching between main player and guest in interactive mode
6. Prevent guest from executing non-movement actions

### Out of scope (first version)
- Dialogue system with guest
- Guest inventory/money/stats
- Guest farming/trading/home menus
- Multiplayer/network sync
- MCP API control of guest (optional future)

---

## 4) UX / Behavior Requirements

### 4.1 Visual identity
- Guest is rendered as a girl emoji (recommended: `👧` or `👩`).
- Player remains existing emoji (`🧑`).

### 4.2 Spawn behavior
- Guest spawns at a safe default tile in Farm (non-blocking, walkable).
- If spawn tile is occupied, use nearest valid walkable fallback tile.

### 4.3 Movement behavior
- Guest movement uses same direction system as player (`up/down/left/right`).
- Guest obeys collision (boundaries, non-walkable tiles, crops/mushrooms if non-walkable).
- Guest can transition through Farm↔East Path connectors.

### 4.4 Action restrictions
When guest is active-controlled:
- Allowed: movement only
- Blocked: clear, plant, water, harvest, buy, sell, sleep, menus
- If blocked command attempted, show message like:
  - `Guest can only walk around.`

### 4.5 Control switching (interactive mode)
Define a key to toggle controlled character (recommended key: `Tab`):
- `Tab`: switch between controlling Player and Guest
- UI footer should show active controller, e.g.:
  - `Control: Player`
  - `Control: Guest`

---

## 5) TUI Mode Requirements

Because this feature is interactive-focused:
- Game must run in interactive TUI mode for this feature path.
- TUI map should display both entities when in same map.
- If both are on same tile (should normally be prevented), rendering priority should be deterministic:
  1) Active-controlled entity
  2) Other entity

Footer/help text should include:
- Arrow keys: Move active character
- Tab: Switch Player/Guest control
- Existing player controls (only when Player active)

---

## 6) Data Model Changes

Add guest fields to `GameState`:
- `guest_enabled: bool`
- `guest_x: usize`
- `guest_y: usize`
- `guest_location: Location`
- `active_control: ControlTarget` (enum: `Player | Guest`)

Optional enum:
```rust
enum ControlTarget {
  Player,
  Guest,
}
```

Persistence:
- Guest fields are included in save/load serialization.

---

## 7) Movement + Transition Rules

### 7.1 Collision
Guest uses existing tile walkability checks.

### 7.2 Transition
When guest crosses transition edge tile:
- update `guest_location`
- set guest spawn/entry coordinate on destination map similar to player transition behavior

### 7.3 Separation rule
Prevent player and guest occupying same tile whenever possible.
If movement would collide with the other entity:
- block movement
- show short message: `Tile occupied.`

---

## 8) Save/Load Behavior

Save file must persist:
- guest existence + position/location
- active control target

On load:
- restore guest state exactly
- validate tile still walkable; if not, relocate to nearest valid tile

---

## 9) API / Runtime Compatibility

### MCP mode
For first version, guest control via MCP is optional and can be excluded.
If excluded, MCP behavior remains unchanged and guest may be hidden/ignored there.

### Time behavior with guest active (required)
When guest mode is active in interactive runtime:
- World time must be **frozen/locked**.
- Guest movement does **not** advance time.
- Player movement/actions in that guest-control interactive session should not tick time until guest mode exits.
- After interactive guest mode exits, MCP/runtime time flow resumes normal rules.

### Interactive mode
Feature is fully supported in interactive TUI path.

---

## 10) Validation Rules

- Guest movement commands ignored when guest not enabled
- Non-movement actions while guest active return blocked message
- Guest cannot open shop/home menus
- Guest cannot mutate economy/farm state

---

## 11) Test Plan

### Unit tests
1. Guest default spawn is valid walkable tile
2. Guest movement respects boundaries/collision
3. Guest map transition Farm↔East Path works
4. Guest cannot execute clear/plant/water/harvest/buy/sell/sleep
5. Control toggle switches active entity
6. Save/load preserves guest state and active control

### Integration tests
1. Interactive session: toggle to guest, walk around farm, transition to East Path
2. Toggle back to player, confirm player actions still work
3. Attempt blocked guest actions and verify messages

### Regression tests
1. Existing player-only controls unchanged
2. Existing MCP behavior unchanged (if guest control not exposed there)

---

## 12) Acceptance Criteria

Feature is complete when:
- Guest girl appears on map in interactive mode
- User can switch control to guest and walk using arrow keys
- Guest can move between Farm and East Path
- Guest cannot perform non-movement actions
- Save/load preserves guest state
- Existing player gameplay remains functional

---

## 13) Resolved Decisions

1. **Enable lifecycle:** Guest is enabled only in interactive mode runtime. When interactive mode exits, guest disappears.
2. **Emoji:** Use `👧` for the guest.
3. **MCP visibility/control:** Guest should be visible in MCP snapshots, but MCP cannot control guest movement/actions.
4. **Collision rule:** Player blocks guest movement (no overlap).
