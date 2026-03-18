# Random Crop Spawn Specification (Spring)

## Overview
Define morning-time random crop spawning rules for Spring, including forced/story spawns and weather-driven spawns.

This system introduces "world-spawned mature crops" that appear on empty grass tiles (`🌿`) at day start.

---

## 1) Goals

1. Add deterministic, testable morning spawn logic for mature crops.
2. Support one-time forced mature flower spawn at Farm `(6,2)` in Spring.
3. Add daily chance-based mature flower spawn (10%).
4. Add rainy-day forced mature mushroom spawn.
5. Keep placement safe (only on valid empty grass tiles).

---

## 2) Non-Goals

1. No new crop growth mechanics in this spec.
2. No spawn rules outside Spring in this phase.
3. No weighted biome rarity tuning beyond rules defined here.
4. No spawning on non-empty or blocked tiles.

---

## 3) Trigger Timing

Spawn evaluation occurs once per new day at **morning/day-start** after day/weather are resolved.

Recommended day-start order:
1. Increment day / wake transition
2. Resolve today weather
3. Apply weather effects (e.g., rain auto-water)
4. Run random-crop spawn rules (this spec)

Must run once per day only.

---

## 4) Spawn Rules (Spring)

## 4.1 Forced Mature Flower at Farm (6,2)

In Spring, mature flower is force-spawned at `Farm (6,2)` when the configured trigger condition is met.

- Tile must be valid for placement (empty grass `🌿`)
- If tile is not empty, do not overwrite existing entity
- Forced-spawn should be tracked to avoid unintended repeats (unless explicitly designed to repeat)

### Trigger definition

"Once something happen" must be wired to a concrete game event flag (implementation-defined), for example:
- first spring day,
- first wake after tutorial,
- first time entering Farm in Spring,
- or explicit scenario flag.

This spec requires: **use a persisted boolean/flag so the force spawn is deterministic and not duplicated unintentionally.**

## 4.2 Daily Mature Flower Chance (10%)

Every Spring day, evaluate a 10% chance to spawn **one** mature flower.

- Chance: 10% (`r in [0,99]`, spawn if `r <= 9`)
- Candidate maps: `Farm` or `EastPath`
- Placement tile: random valid empty grass tile (`🌿`) in chosen map
- If no valid tile exists in chosen map, fallback to the other map
- If no valid tile exists in both maps, no spawn that day

## 4.3 Rainy-Day Forced Mature Mushroom

On each rainy Spring day, force spawn **one** mature mushroom.

- Condition: today weather is rainy
- Candidate maps: `Farm` or `EastPath`
- Placement tile: random valid empty grass tile (`🌿`)
- If chosen map has no valid tile, fallback to other map
- If no valid tile in both maps, skip safely

Rainy-day mushroom spawn is independent from daily flower chance; both may occur on same day.

---

## 5) Placement & Safety Rules

A tile is spawn-eligible if all are true:
1. Tile base is empty grass (`🌿`)
2. No crop/entity currently occupies tile
3. Tile is not blocked/reserved by static object or map rule

No overwrite policy:
- Never replace existing crops/objects/entities.

Spawn count cap (recommended):
- At most 1 daily chance flower + 1 rainy mushroom + optional forced scripted flower event per day transition.

---

## 6) Determinism & RNG

1. Use seeded RNG path.
2. Given same seed and same day-transition sequence, spawn outcomes must be reproducible.
3. Keep RNG draw order stable and documented:
   - flower chance roll
   - map/tile selection
   - rainy mushroom map/tile selection
4. Avoid hidden RNG draws that alter spawn sequence unexpectedly.

---

## 7) Data Model

Suggested persisted fields:

- `spring_forced_flower_6_2_done: bool` (or equivalent event flag)
- `last_spawn_processed_day: u32` (guard against double-processing same day)
- existing `weather` / `day` fields reused

No schema migration required if defaults are backward-compatible.

---

## 8) Save/Load Behavior

1. Mid-day load must not re-run morning spawn logic.
2. Loading on a new unprocessed day should run day-start logic exactly once.
3. Forced flower event flag must persist across saves.

---

## 9) MCP / API Behavior

If MCP state/map endpoints are available:

1. Spawned mature crops should be visible in map/state after day start.
2. Day-start events should include spawn events for clients/debugging.
3. Deterministic behavior should hold under MCP-driven day progression.

Recommended event payload examples:
- `SpawnedMatureFlower { map, x, y, source }`
- `SpawnedMatureMushroom { map, x, y, source: rainy_day }`

---

## 10) Edge Cases

1. **No empty grass tiles available:** skip spawn safely.
2. **Farm (6,2) occupied during forced event:** do not overwrite; optionally log `blocked` result.
3. **Rainy day + flower chance success:** both spawns may happen if valid tiles exist.
4. **Multiple day jumps in one operation:** process day-start spawns per day in sequence or document current behavior and keep consistent.
5. **Re-entry/map change same day:** must not trigger new morning spawn roll.

---

## 11) Testing Plan

## 11.1 Unit Tests

1. Forced flower at `(6,2)` spawns only when trigger flag condition is true.
2. Forced flower does not overwrite occupied tile.
3. Flower chance 10% threshold logic is correct.
4. Rainy-day mushroom spawn executes only on rainy weather.
5. Eligibility filter accepts only empty grass (`🌿`) tiles.

## 11.2 Integration Tests

1. Spring morning transition executes spawn pipeline once.
2. Rainy day produces mature mushroom when valid tile exists.
3. Daily flower appears over many days near expected frequency (~10%).
4. Save/load does not duplicate same-day spawns.
5. Spawned crops are harvestable as mature crops.

## 11.3 Property/Statistical Tests

Over large Spring sample (e.g., 100,000 days):
- Daily chance flower occurrence approximates 10% (tolerance band, e.g., ±0.5% to ±1.0%).

---

## 12) Rollout Plan

1. Add day-start spawn coordinator.
2. Implement forced flower `(6,2)` event flag + placement.
3. Implement daily 10% mature flower spawn.
4. Implement rainy-day mature mushroom forced spawn.
5. Add events/logging and tests.
6. Validate save/load idempotency.

---

## 13) Acceptance Criteria

Feature is complete when:

1. In Spring, configured forced event can spawn a mature flower at `Farm (6,2)` without overwriting occupied tile.
2. Each Spring day evaluates a 10% chance for one mature flower spawn on empty grass in Farm/EastPath.
3. Each rainy Spring day force-spawns one mature mushroom on empty grass in Farm/EastPath.
4. Rainy mushroom spawn and daily flower spawn can both occur on the same day.
5. Spawns run once per day-start and remain consistent across save/load.
6. No spawn ever replaces non-empty tiles.
