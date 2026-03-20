# Spring Wonder Implementation Plan

Source:
- `agents/spring-wonder.spec.md`

## Scope
Implement Spring Wonder event using butterfly tile (`🦋`) at Square `(2,2)` on Spring day 28, with player-facing message and MCP visibility.

---

## Phase 0 — Baseline
1. Run baseline checks:
   - `cargo test`
   - `cargo clippy --all-targets --all-features -- -D warnings`
2. Confirm current Square coordinates and movement path around `(2,2)`.

Deliverable:
- Clean baseline and known-good map assumptions.

---

## Phase 1 — Tile Model
1. Add `TileType::Wonder` in `src/world.rs`.
2. Add emoji mapping:
   - `Wonder -> 🦋`
3. Set walkability:
   - Wonder must be non-walkable.

Deliverable:
- Core tile model supports Wonder.

---

## Phase 2 — Day-28 Spawn Logic
1. Add Wonder spawn hook in day-start flow (`start_new_day` path in `src/state.rs`).
2. Spawn condition:
   - `season == "Spring" && day == 28`
3. Spawn target:
   - `square_map[2][2] = TileType::Wonder`
4. Ensure this doesn’t break existing day-start steps:
   - crop growth,
   - soil reset,
   - bubble reset,
   - random spawn flow.

Deliverable:
- Deterministic Wonder appearance on Spring day 28.

---

## Phase 3 — Player Interaction Message
1. In player movement flow, detect when movement is blocked by `TileType::Wonder`.
2. Set message exactly:
   - `That is so beautiful. Let human enjoy it together in interactive mode.`
3. Ensure Wonder remains non-walkable and movement is rejected.

Deliverable:
- Wonder interaction shows intended narrative text while blocking entry.

---

## Phase 4 — MCP / Serialization
1. Update MCP compact map encoding:
   - add Wonder symbol (e.g., `W`).
2. Update MCP detailed tile names:
   - add `"Wonder"`.
3. Validate both serializers:
   - `src/mcp/session.rs`
   - `src/mcp/state_manager.rs`

Deliverable:
- MCP clients can see and reason about Wonder tile.

---

## Phase 5 — Tests
Add/adjust tests:
1. `test_wonder_spawns_on_spring_day_28_at_square_2_2`
2. `test_wonder_tile_renders_butterfly_emoji`
3. `test_wonder_tile_is_walkable`
4. `test_wonder_message_on_player_step`
5. `test_mcp_map_encodes_wonder_tile`

Run full validation:
- `cargo fmt`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

Deliverable:
- Green suite with Wonder coverage.

---

## Phase 6 — Manual Verification
1. Advance to Spring day 28.
2. Enter Square and inspect `(2,2)` tile.
3. Confirm `🦋` renders in UI.
4. Attempt to step onto tile, confirm movement is blocked and message text appears.
5. Verify clear/plant/harvest/fishing actions do not affect Wonder tile.
6. Verify MCP `print/getMap` includes Wonder tile encoding/name.

Deliverable:
- End-to-end confirmation in gameplay + MCP.

---

## Suggested Commit Sequence
1. `feat(world): add Wonder tile type and butterfly rendering`
2. `feat(state): spawn Wonder on Spring day 28 in Square`
3. `feat(ui): show Wonder interaction message on player step`
4. `feat(mcp): expose Wonder in map serialization`
5. `test(wonder): add Spring Wonder coverage`
6. `docs(agents): add Spring Wonder implementation plan`
