# Fishing Action Spec

## Goal
Add a new player action: **fishing**.

Fishing allows player to fish at nearby river tiles and potentially obtain fish items for inventory/sale.

---

## Feature Summary

- New command/action: `fishing`
- Player can fish only when near a valid river tile (`🌊`).
- Fishing consumes **1 in-game hour**.
- Fishing outcome probabilities:
  - **20%** → `🐟` (value: **$80**)
  - **70%** → nothing
  - **10%** → `🐠` (value: **$180**)
- `🐟` and `🐠` are inventory items and can be sold.
- On fish success (`🐟` or `🐠`), show a congratulatory bottom message.

---

## Action Rules

### 1) Valid fishing context
Player can fish only when:
- current region supports river fishing (initially South River), and
- at least one adjacent tile (up/down/left/right) is a valid river target (`🌊` or `🫧`).

Fishing should not depend on player facing direction.
The action should auto-select a valid adjacent target tile using deterministic priority order:
1. Up
2. Right
3. Down
4. Left

If no valid river tile is in fishing range:
- action fails with a clear message (e.g., “No river nearby to fish.”),
- no time passes.

### 2) Time cost
- Each fishing attempt consumes **60 in-game minutes**.
- Any day/night/auto-sleep logic must still apply after time advancement.

### 2.1) River bubble state after fishing
- The specific auto-selected adjacent river tile used for fishing should change from river (`🌊`) to **river bubble** (`🫧`).
- River bubble tiles are still valid fishing targets.
- River bubble tiles are non-walkable (same as river).
- On sleep cycle completion (new day wake-up), all `🫧` tiles reset back to `🌊`.

### 3) Outcome probabilities
Per fishing attempt:
- 0.20 chance: add `FishCommon` (`🐟`) to inventory.
- 0.70 chance: add nothing.
- 0.10 chance: add `FishRare` (`🐠`) to inventory.

Probability resolution should use existing deterministic/random framework used by game actions.

### 4) Success/failure messaging
- On `🐟` or `🐠` success, show congratulatory bottom text, for example:
  - `🎉 Nice catch! You got 🐟`
  - `🎉 Amazing catch! You got 🐠`
- On nothing outcome, show neutral text, for example:
  - `No bite this time.`

---

## Data Model Changes

### Inventory
Extend inventory with fish buckets, for example:
- `fish: HashMap<FishType, u32>`

### New FishType enum
Add fish item enum:
- `FishCommon` (emoji `🐟`, sell price `$80`)
- `FishRare` (emoji `🐠`, sell price `$180`)

### Tile additions for fishing state
Add/extend map tile semantics:
- `River` (`🌊`, non-walkable)
- `RiverBubble` (`🫧`, non-walkable, fishable)

### Selling integration
- Selling flow/menu must support fish items.
- Sale updates money and daily income tracking like existing produce/forage sales.

---

## UI / Rendering

### Snapshot/print UI
- Inventory section should display fish counts, e.g.:
  - `Fish: 🐟 x2`
  - `Fish: 🐠 x1`
- Bottom message area should display fishing result text.

### Interactive mode
- Fishing support in interactive mode is optional and not required for this scope.
- If omitted, no interactive key binding/menu change is needed.

---

## MCP / Commands

Add command parsing support:
- `fishing`
- Directional variant `fishing:<dir>` is optional/legacy; default `fishing` should work without requiring facing logic.

Command response should include:
- message,
- event (`Fishing attempt`, `Caught 🐟`, etc.),
- state delta updates for time/inventory/money when relevant.

---

## Economy Rules

- `🐟` sell value: **$80**
- `🐠` sell value: **$180**
- No direct money gain from fishing itself; money only changes when sold.

---

## Acceptance Criteria

1. Player can fish when any adjacent tile (up/down/left/right) is a river-like tile (`🌊` or `🫧`), without relying on facing direction.
2. Each fishing attempt advances time by 1 hour.
3. Outcome probabilities match 20% / 70% / 10%.
4. Fish items (`🐟`, `🐠`) are stored in inventory.
5. Fish items can be sold for correct values ($80 / $180).
6. Success catches display congratulatory bottom text.
7. MCP mode supports fishing action (interactive mode fishing is not required).
8. After fishing, targeted river tile changes to `🫧` and remains fishable.
9. Sleep cycle resets all `🫧` tiles back to `🌊`.
10. Save/load preserves fish inventory and river bubble state.

---

## Suggested Tests

- `test_fishing_requires_nearby_river`
- `test_fishing_advances_time_by_one_hour`
- `test_fishing_outcome_common_fish`
- `test_fishing_outcome_rare_fish`
- `test_fishing_outcome_nothing`
- `test_fishing_adds_items_to_inventory`
- `test_fish_sell_values`
- `test_fishing_success_shows_congrats_message`
- `test_fish_inventory_persists_save_load`
- `test_river_tile_turns_into_bubble_after_fishing`
- `test_river_bubble_tile_is_still_fishable`
- `test_sleep_cycle_resets_bubble_to_river`
- `test_river_bubble_state_persists_save_load`
- `test_mcp_parse_fishing_command`

---

## Non-Goals (for first iteration)

- No fishing rod/tool durability.
- No weather/time-of-day modifiers on fish rates.
- No region-specific fish table beyond current two fish outcomes.
