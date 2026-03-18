# Four-Specs Implementation Plan

**Generated:** 2026-03-18  
**Specs:** `heartbeat.spec.md`, `day-night.spec.md`, `auto-sleep.spec.md`, `random-weather.spec.md`

---

## Overview

These four specs introduce time-based game mechanics to Shelldew:
1. **Heartbeat**: Continuous time flow (1 real sec = 5 game min), pause integration, save/load
2. **Day-Night**: Visual day/night state (06:00-17:59 day, 18:00-05:59 night), moon icon at night
3. **Auto-Sleep**: Force sleep at 02:00, use existing sleep pipeline
4. **Random-Weather**: Spring weather (80% Sunny, 13% Cloudy, 7% Rainy), rainy auto-waters crops

### Runtime Clarification (Observed)

- Heartbeat currently behaves as wall-clock driven in interactive loop context.
- In the current MCP request/response flow, time mostly advances on command-driven actions unless explicit MCP idle ticking is implemented.

---

## Architecture Changes by Module/File

### 1. `src/state.rs` (Core Game State)

| Change | Description |
|--------|-------------|
| **New field:** `total_minutes: u32` | Total in-game minutes since start (replaces `hour`/`minute` for internal use) |
| **New field:** `last_update_ms: u64` | Wall-clock timestamp of last heartbeat tick |
| **New field:** `is_paused: bool` | Pause state for heartbeat |
| **New field:** `weather: Weather` | Replace `String` with typed enum |
| **New field:** `weather_day: u32` | Day number weather is bound to |
| **New field:** `auto_sleep_triggered_day: u32` | Guard to prevent duplicate triggers |
| **New field:** `rng_seed: u64` | Seeded RNG for deterministic weather |
| **Modify:** `start_new_day()` | Add weather roll with seeded RNG, apply rain watering |
| **New:** `tick(current_time_ms: u64)` | Heartbeat tick function |
| **New:** `should_auto_sleep() -> bool` | Auto-sleep trigger condition |
| **New:** `run_auto_sleep()` | Execute auto-sleep flow |
| **New:** `is_day() -> bool` | Day/night helper |
| **New:** `get_day_and_time() -> (u32, u8, u8)` | Convert total_minutes to day/hour/minute |

### 2. `src/world.rs` (World Definitions)

| Change | Description |
|--------|-------------|
| **New enum:** `Weather { Sunny, Cloudy, Rainy }` | Replace String weather |
| **Modify:** `TileType::emoji()` | No changes needed (already handles crop states) |

### 3. `src/mcp/session.rs` (Session Management)

| Change | Description |
|--------|-------------|
| **Modify:** `to_snapshot()` | Include `total_minutes`, `is_paused`, `weather` in JSON |
| **Modify:** `to_stats()` | Add heartbeat telemetry fields |

### 4. `src/mcp/handler.rs` (MCP Handler)

| Change | Description |
|--------|-------------|
| **New:** Handle `get_world_time` action | Return `{total_minutes, hour, minute, day}` |

### 5. `src/main.rs` (Entry Point)

| Change | Description |
|--------|-------------|
| **New:** Heartbeat loop (100ms interval) | Use `std::time::Instant` for wall-clock timing |
| **Modify:** `handle_input()` | Pause/unpause handling |
| **Modify:** `render()` / `print_header()` | Show time with pause indicator |

---

## Ordered Implementation Phases

### Phase 1: Foundation - GameTime & Heartbeat
**Goal:** Implement core time system

1. Add `total_minutes`, `last_update_ms`, `is_paused` to `GameState`
2. Implement `tick(current_time_ms)` with:
   - 100ms interval → 0.5 game minutes
   - 5-second max catch-up cap
   - Pause guard
3. Add `get_day_and_time()` helper
4. Wire heartbeat to main loop
5. Add pause integration (ESC key, menu open/close)
6. Include `total_minutes` in save/load
7. Add UI time display `[HH:MM]` with pause indicator

**Dependencies:** None

### Phase 2: Day-Night System
**Goal:** Visual day/night state

1. Add `is_day()` helper (06:00-17:59 = day)
2. Add `get_display_weather_icon()` helper
3. Modify `get_weather_icon()` to:
   - Return `🌙` when night
   - Return weather icon when day
4. Add unit tests for boundary times

**Dependencies:** Phase 1 (uses `hour`/`minute`)

### Phase 3: Auto-Sleep System
**Goal:** Force sleep at 02:00

1. Add `auto_sleep_triggered_day` guard field
2. Add `should_auto_sleep()` checker
3. Add `run_auto_sleep()` that:
   - Locks free-roam input
   - Calls existing sleep pipeline (`perform_sleep()`)
   - Shows income summary
   - Wakes at "front of home" (coordinates 3,3)
4. Integrate trigger check in heartbeat tick
5. Handle edge cases: time jumps, pause at 02:00, menu interaction

**Dependencies:** Phase 1 (needs tick point)

### Phase 4: Random Weather System
**Goal:** Spring weather with auto-watering

1. Add `Weather` enum in `world.rs`
2. Replace `weather: String` with `weather: Weather`
3. Add `rng_seed` field for deterministic replay
4. Implement `roll_weather()`:
   - Use seeded RNG
   - 80% Sunny (0-79), 13% Cloudy (80-92), 7% Rainy (93-99)
5. Modify `start_new_day()`:
   - Call `roll_weather()` if `weather_day != day`
   - If Rainy, apply auto-watering to all eligible crops
6. Ensure idempotency (check `watered_today` before setting)
7. Modify `get_weather_icon()` to use enum

**Dependencies:** Phase 2 (needs day/night icon priority)

---

## Dependency Graph

```
Phase 1: Heartbeat
    ├── Add GameTime fields to state.rs
    ├── tick() implementation
    ├── Pause integration
    └── Save/load + MCP time query
    │
    ▼
Phase 2: Day-Night  [depends on Phase 1]
    ├── is_day() helper
    ├── get_display_weather_icon()
    └── Weather icon rendering
    │
    ▼
Phase 3: Auto-Sleep  [depends on Phase 1]
    ├── should_auto_sleep()
    ├── run_auto_sleep()
    └── Tick integration
    │
    ▼
Phase 4: Random-Weather  [depends on Phase 2]
    ├── Weather enum
    ├── roll_weather() with seeding
    ├── Rain auto-watering
    └── Icon rendering priority
```

**Note:** Auto-sleep (Phase 3) depends only on Phase 1, not Phase 2, because it doesn't need day/night logic—it triggers at 02:00 (always night).

---

## Risk List

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Heartbeat timing drift** | Medium | Use wall-clock `Instant`, cap catch-up at 5s |
| **MCP time queries during tick** | Low | Return current `total_minutes`, not wall time |
| **Auto-sleep during menu** | Medium | Check trigger on resume, not just tick |
| **Weather seed persistence** | Medium | Store seed in save, derive sequence from day |
| **Rain double-watering** | Low | Check `watered_today` before applying |
| **Batch commands crossing 02:00** | Medium | Auto-sleep resolves before continuing batch |
| **Day wrap with large time jump** | Low | Use modulo for day calculation |

---

## Test Matrix

### Unit Tests

| Test | Spec | Phase |
|------|------|-------|
| `tick_1_second_advances_5_minutes` | Heartbeat | 1 |
| `tick_10_seconds_advances_50_minutes` | Heartbeat | 1 |
| `pause_stops_time` | Heartbeat | 1 |
| `save_load_preserves_time` | Heartbeat | 1 |
| `day_wrap_at_1440_minutes` | Heartbeat | 1 |
| `lag_spike_capped_at_5_seconds` | Heartbeat | 1 |
| `is_day_true_06_to_17` | Day-Night | 2 |
| `is_night_true_18_to_05` | Day-Night | 2 |
| `night_icon_is_moon` | Day-Night | 2 |
| `day_icon_is_weather` | Day-Night | 2 |
| `should_auto_sleep_at_02_00` | Auto-Sleep | 3 |
| `should_not_auto_sleep_at_01_59` | Auto-Sleep | 3 |
| `auto_sleep_triggers_once_per_day` | Auto-Sleep | 3 |
| `weather_80_13_7_distribution` | Weather | 4 |
| `rainy_day_waters_crops` | Weather | 4 |
| `weather_idempotent_same_day` | Weather | 4 |
| `weather_preserved_after_save_load` | Weather | 4 |

### Integration Tests

| Test | Spec | Phase |
|------|------|-------|
| Start game, verify clock shows 06:00 | Heartbeat | 1 |
| Wait 60 real seconds, verify clock shows 11:00 | Heartbeat | 1 |
| Open menu at 12:00, wait 30s, close, verify still 12:00 | Heartbeat | 1 |
| Advance to 06:00, verify day icon | Day-Night | 2 |
| Advance to 18:00, verify moon icon | Day-Night | 2 |
| Save at night, reload, verify moon icon | Day-Night | 2 |
| Simulate time to 02:00, verify sleep flow | Auto-Sleep | 3 |
| Verify income summary appears | Auto-Sleep | 3 |
| Verify wake at front of home | Auto-Sleep | 3 |
| Day rollover displays new weather icon | Weather | 4 |
| Rainy day auto-waters crops, verify growth | Weather | 4 |
| Save/load rainy day preserves watered state | Weather | 4 |

### MCP Tests

| Test | Spec | Phase |
|------|------|-------|
| `get_world_time` returns correct time | Heartbeat | 1 |
| `getState` includes weather field | Weather | 4 |
| Command at 01:59, next tick 02:00 triggers auto sleep | Auto-Sleep | 3 |
| Batch crossing 02:00 stops and resolves sleep | Auto-Sleep | 3 |

---

## MCP Impact

### New MCP Actions

```json
{
  "action": "get_world_time"
}
// Returns: { "total_minutes": 720, "hour": 12, "minute": 0, "day": 2 }
```

### Modified MCP Responses

| Endpoint | Change |
|----------|--------|
| `getState` | Add `weather` (enum string), `total_minutes`, `is_paused` |
| `getStats` | Add `heartbeat.tick_count`, `heartbeat.pause_duration_ms` |
| `command` result | Add `sleep_event` flag when auto-sleep triggers |
| `commandBatch` result | Add `stopped_due_to_sleep` flag |

### Session Snapshot Changes

```json
{
  "day": 2,
  "hour": 12,
  "minute": 0,
  "total_minutes": 720,
  "weather": "Sunny",
  "weather_day": 2,
  "is_paused": false
}
```

---

## Rollout Strategy

### Step 1: Shadow Mode (not applicable - no AI)
- Implement Phase 1-4 in feature branch
- Run all unit tests locally
- Run integration tests

### Step 2: Internal Testing
- Merge to main after code review
- Run `cargo test` (all tests pass)
- Run `cargo clippy` (no warnings)
- Run existing integration tests

### Step 3: Staged Rollout
- Deploy to MCP-compatible client
- Test all MCP interactions
- Verify save/load round-trip

### Step 4: Monitoring
- Track heartbeat telemetry (if available)
- Verify no crash reports related to time/weather

---

## Acceptance Mapping

### Heartbeat Spec

| Criterion | Implementation |
|-----------|----------------|
| 1 real sec = 5 game min | `tick()` advances 0.5 min per 100ms |
| Pause integration | `is_paused` flag stops tick |
| Save/load | `total_minutes` serialized |
| Lag tolerance | 5-second cap on catch-up |
| MCP compatibility | `get_world_time` action |

### Day-Night Spec

| Criterion | Implementation |
|-----------|----------------|
| Day period 06:00-17:59 | `is_day()` returns `hour >= 6 && hour < 18` |
| Night period 18:00-05:59 | `is_night()` returns `!is_day()` |
| Weather icon = 🌙 at night | `get_weather_icon()` checks `is_night()` first |
| Daytime weather unchanged | Fall through to weather enum icons |
| Save/load consistency | Re-derived from time on load |

### Auto-Sleep Spec

| Criterion | Implementation |
|-----------|----------------|
| Auto-fall asleep at 02:00 | `should_auto_sleep()` checks `hour == 2 && minute == 0` |
| Uses normal sleep pipeline | Calls `perform_sleep()` → `HomeState::Income` |
| Income summary shown | Handled by existing `HomeState::Income` flow |
| Wake at front of home | Set `player_x=3, player_y=3` on day transition |
| Once per day | `auto_sleep_triggered_day` guard |

### Random-Weather Spec

| Criterion | Implementation |
|-----------|----------------|
| 80% Sunny, 13% Cloudy, 7% Rainy | Weighted selection with thresholds |
| Weather selected once per day | Check `weather_day != day` in `start_new_day()` |
| Rain auto-waters crops | Iterate crops, set `watered_today = true` |
| Daytime UI shows weather icon | `get_weather_icon()` returns enum icon during day |
| Night icon = 🌙 | Day-night spec handles priority |
| Save/load preserves weather | `weather` and `weather_day` serialized |
| Seeded RNG for determinism | `rng_seed` field, derived from initial seed + day |

---

## Implementation Notes

1. **No new crate dependencies** - All functionality uses std library (`std::time::Instant`)
2. **Backward compatibility** - Existing `hour`/`minute` computed from `total_minutes` for display
3. **Single source of truth** - All time/weather/day-night logic via helper functions
4. **Idempotent operations** - Rain watering checks `!watered_today` before applying
5. **Guard pattern** - Auto-sleep and weather re-roll use day-number guards
