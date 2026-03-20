# Fishing Implementation Plan

## Scope
Implement fishing in MCP mode (interactive fishing not required), following `agents/fishing.spec.md`.

Core requirements:
- Add new `fishing` action/command.
- Allow fishing only when near river-like tiles.
- Fishing costs 1 in-game hour.
- Outcome table:
  - 20% `🐟` (sell $80)
  - 70% nothing
  - 10% `🐠` (sell $180)
- On fish success, show congratulatory bottom message.
- After fishing, targeted `🌊` becomes `🫧` (river bubble).
- `🫧` remains fishable and non-walkable.
- Sleep cycle resets all `🫧` back to `🌊`.
- Fish inventory persists and fish can be sold.

---

## Phase 0 — Baseline & Safety

1. Run baseline tests.
2. Verify current river tile behavior in SouthRiver/EastPath.
3. Confirm current sleep-cycle hook used for daily reset.

Deliverable:
- Stable baseline before fishing changes.

---

## Phase 1 — Data Model Extensions

### 1.1 Fish item model
- Add `FishType` enum:
  - `FishCommon` (`🐟`, value 80)
  - `FishRare` (`🐠`, value 180)

### 1.2 Inventory model
- Add `fish: HashMap<FishType, u32>` to inventory.
- Add helpers:
  - `add_fish(...)`
  - `sell_fish(...)`
  - optional `fish_count(...)`

### 1.3 Tile model
- Add `TileType::RiverBubble`.
- Rendering emoji: `🫧`.
- Movement: non-walkable.
- Fishable: yes.

Deliverable:
- Types compile with fish + bubble support.

---

## Phase 2 — Fishing Action Logic (MCP)

### 2.1 Command parser
- Add `fishing` command parsing.
- Optional directional variant can be deferred unless needed.

### 2.2 Valid target rules
- Fishing allowed when any adjacent tile (up/right/down/left) is:
  - `River` or `RiverBubble`.
- Do not require facing direction for default `fishing` command.
- Auto-select target tile deterministically with priority:
  1. Up
  2. Right
  3. Down
  4. Left
- If invalid target:
  - return “No river nearby to fish.” (or equivalent),
  - no time advancement.

### 2.3 Resolve outcome
- Use existing RNG approach in state.
- Apply probabilities exactly 20/70/10.
- On success:
  - add fish to inventory,
  - show congratulatory message.
- On miss:
  - show neutral message.

### 2.4 Bubble mutation
- If auto-selected fishing target tile is `River`, convert to `RiverBubble`.
- If already `RiverBubble`, keep as `RiverBubble`.

### 2.5 Time cost
- Advance game time by 60 minutes per valid fishing attempt.

Deliverable:
- MCP `fishing` command works end-to-end.

---

## Phase 3 — Selling Integration

1. Extend sell command/menu logic to support fish sales.
2. Apply values:
   - `🐟` = $80
   - `🐠` = $180
3. Track sale in income/accounting structures.

Deliverable:
- Fish can be sold and money updates correctly.

---

## Phase 4 — Sleep Reset for Bubble Tiles

1. On sleep-cycle/day reset path, iterate river maps.
2. Replace all `RiverBubble` -> `River`.
3. Ensure reset happens once per sleep cycle/day transition.

Deliverable:
- Bubble reset behavior is deterministic and complete.

---

## Phase 5 — MCP/UI Output Integration

### 5.1 Snapshot inventory output
- Include fish inventory lines in print snapshot.

### 5.2 Tile serialization
- `getMap` and MCP detailed tile outputs include `RiverBubble`.

### 5.3 Messaging
- Ensure fishing result text appears in bottom message area.

Deliverable:
- MCP output clearly reflects fishing and bubble state.

---

## Phase 6 — Persistence

1. Save/load includes:
   - fish inventory,
   - `RiverBubble` tile states.
2. Validate backward compatibility for old saves where possible.

Deliverable:
- State roundtrip fully preserves fishing-related data.

---

## Phase 7 — Tests

Add tests (or equivalent coverage):
- `test_fishing_requires_nearby_river`
- `test_fishing_advances_time_by_one_hour`
- `test_fishing_outcome_common_fish`
- `test_fishing_outcome_rare_fish`
- `test_fishing_outcome_nothing`
- `test_fishing_adds_items_to_inventory`
- `test_fish_sell_values`
- `test_fishing_success_shows_congrats_message`
- `test_river_tile_turns_into_bubble_after_fishing`
- `test_river_bubble_tile_is_still_fishable`
- `test_sleep_cycle_resets_bubble_to_river`
- `test_river_bubble_state_persists_save_load`
- `test_mcp_parse_fishing_command`

Deliverable:
- Green test suite with fishing+bubble coverage.

---

## Phase 8 — Manual Verification

Manual MCP script flow:
1. Move player to SouthRiver edge.
2. Run `fishing` repeatedly.
3. Observe bubble conversion on auto-selected adjacent river tiles.
4. Confirm fish inventory changes on catches.
5. Sell fish and verify money deltas.
6. Trigger sleep cycle and verify all `🫧` reset to `🌊`.

Deliverable:
- Manual evidence of all acceptance criteria.

---

## Suggested Commit Sequence

1. `feat(fishing): add fish inventory model and fish types`
2. `feat(tiles): add RiverBubble tile state and rendering`
3. `feat(mcp): add fishing command with time cost and probability outcomes`
4. `feat(economy): support fish selling values and income tracking`
5. `fix(reset): reset RiverBubble tiles on sleep cycle`
6. `test(fishing): add fishing+bubble unit and MCP coverage`
7. `docs(fishing): align spec and implementation notes`
