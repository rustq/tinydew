# Game Heartbeat Behavior Specification

## Overview

This spec defines the time advancement system for Shelldew's game world. Every 1 real-world second advances world time by 5 in-game minutes.

### Current Runtime Note

- **Interactive runtime loop:** heartbeat is wall-clock driven (`tick()` over real time).
- **Current MCP request/response flow:** time is effectively command-driven unless MCP idle ticking is explicitly wired.

--------------------------------------------------------------------

# 1. Goals

- **Consistent time flow**: 1 real-time second = 5 in-game minutes (300 in-game seconds)
- **Seamless pause integration**: Heartbeat stops/resumes correctly with game pause state
- **Accurate save/load**: Game time persists across save/load cycles with correct elapsed time
- **Lag tolerance**: System handles frame drops and processing delays without significant time drift
- **MCP compatibility**: External time queries return accurate world time

--------------------------------------------------------------------

# 2. Non-Goals

- **Variable time scales**: No speed-up/slow-down time controls in MVP
- **Real-time calendar**: Date/year progression is derived from accumulated minutes only
- **Network sync**: No multiplayer time synchronization
- **Background time**: Time does not advance when game is not running (real-time persistence)

--------------------------------------------------------------------

# 3. Timing Model

## 3.1 Core Constants

```
REAL_TIME_SECONDS_PER_GAME_MINUTE = 0.2  (1 second / 5 minutes)
GAME_MINUTES_PER_REAL_SECOND = 5
GAME_SECONDS_PER_REAL_SECOND = 300  (5 min × 60 sec)
```

## 3.2 Time Representation

- **World time stored as**: Total in-game minutes since game start (integer)
- **Display format**: `HH:MM` (24-hour clock)
- **Day boundary**: 24:00 (1440 minutes) wraps to next day

## 3.3 Heartbeat Implementation

```rust
// Pseudocode structure
struct GameTime {
    total_minutes: u32,      // Total in-game minutes elapsed
    last_update_ms: u64,     // Timestamp of last heartbeat tick
    is_paused: bool,
}

impl GameTime {
    fn tick(&mut self, current_time_ms: u64) {
        if self.is_paused { return; }

        let elapsed_ms = current_time_ms - self.last_update_ms;
        let elapsed_seconds = elapsed_ms / 1000;

        // 1 real second = 5 game minutes
        let minutes_advanced = elapsed_seconds * GAME_MINUTES_PER_REAL_SECOND;
        self.total_minutes += minutes_advanced;
        self.last_update_ms = current_time_ms;
    }
}
```

## 3.4 Update Frequency

- Heartbeat fires every **100ms** (10 ticks/second)
- Each tick advances 0.5 in-game minutes (30 seconds game time)
- Finer granularity reduces visual stutter; coarser is acceptable but 100ms is recommended

--------------------------------------------------------------------

# 4. Pause/Menu Behavior

## 4.1 Pause State

- When game enters pause (ESC key or menu open): `is_paused = true`
- Heartbeat **stops** incrementing time when paused
- `last_update_ms` does NOT advance during pause

## 4.2 Menu Interactions

- Opening any menu (inventory, crafting, etc.) triggers pause
- Time freezes immediately upon menu open
- Time resumes from exact point when menu closes

## 4.3 Resume Behavior

```rust
fn resume(&mut self, current_time_ms: u64) {
    self.is_paused = false;
    self.last_update_ms = current_time_ms;  // Reset to avoid time jump
}
```

--------------------------------------------------------------------

# 5. Save/Load Interactions

## 5.1 Save Data

Save file includes:
```rust
struct SavedGameTime {
    total_minutes: u32,
    last_save_timestamp_ms: u64,  // Real-time of save (for offline calc if needed)
}
```

## 5.2 Load Behavior

- On load, `total_minutes` restored directly from save
- `last_update_ms` set to current real-time
- No "catch-up" time applied (player starts at saved time immediately)

## 5.3 Offline Time (Non-Goal)

Not implemented in MVP. Future consideration: real-time elapsed since last save could advance time while away.

--------------------------------------------------------------------

# 6. MCP Implications

## 6.1 Time Queries

MCP tools can query current world time via:
```json
{
  "action": "get_world_time"
}
```

Returns:
```json
{
  "total_minutes": 720,
  "hour": 12,
  "minute": 0,
  "day": 2
}
```

## 6.2 MCP-Initiated Actions

- If MCP sends an action that takes simulated time (e.g., "sleep until morning"), calculate new time and update `total_minutes` directly
- MCP should NOT call tick() directly; use explicit time mutations

--------------------------------------------------------------------

# 7. Edge Cases

## 7.1 Lag Handling

- **Frame drops**: Accumulate elapsed time each frame; don't skip if behind
- **Tab-out/throttle**: Use wall-clock time (`std::time::Instant`) not frame counting
- **Max catch-up**: If >5 real seconds elapse in single tick, cap at 5 seconds worth of game time (prevents time jumps on lag spike)

```rust
const MAX_ELAPSED_SECONDS: u64 = 5;
let elapsed_seconds = min(elapsed_seconds, MAX_ELAPSED_SECONDS);
```

## 7.2 Drift Correction

- Minor drift (<1 second real-time) is acceptable
- No explicit NTP-style sync needed for MVP
- Drift only matters for long play sessions; acceptable to reset on save/load

## 7.3 Time Overflow

- `total_minutes` uses `u32` (max ~4 billion minutes ≈ 2,850 years)
- Safe for practical use; no overflow handling needed

## 7.4 Day Wrap

```rust
fn get_day_and_time(minutes: u32) -> (day: u32, hour: u8, minute: u8) {
    let day = minutes / 1440 + 1;           // Days start at 1
    let minute_of_day = minutes % 1440;
    let hour = (minute_of_day / 60) as u8;
    let minute = (minute_of_day % 60) as u8;
    (day, hour, minute)
}
```

--------------------------------------------------------------------

# 8. Testing Plan

## 8.1 Unit Tests

| Test | Expected Behavior |
|------|-------------------|
| `tick_1_second` | After 1000ms elapsed, total_minutes += 5 |
| `tick_10_seconds` | After 10000ms elapsed, total_minutes += 50 |
| `pause_stops_time` | Pause → tick 5s → unpause → no time advanced |
| `save_load_preserves_time` | Save at 720min → load → total_minutes == 720 |
| `day_wrap` | 1440 minutes → day=2, hour=0, minute=0 |

## 8.2 Integration Tests

- Start game, verify clock shows 6:00 (default start time)
- Wait 60 real seconds, verify clock shows 11:00
- Open menu at 12:00, wait 30 real seconds, close menu, verify still 12:00

## 8.3 Edge Case Tests

- Simulate 10-second lag spike, verify time advances max 5 seconds worth
- Rapid pause/unpause 10x, verify no time leakage

--------------------------------------------------------------------

# 9. Rollout Notes

## 9.1 Implementation Order

1. Add `GameTime` struct with `total_minutes` and `last_update_ms`
2. Implement `tick()` using `std::time::Instant`
3. Wire heartbeat to main game loop (100ms interval)
4. Add pause integration to `tick()` and `resume()`
5. Include `total_minutes` in save/load serialization
6. Expose world time to MCP if MCP integration exists
7. Add UI display for current time (top-right corner)

## 9.2 Default Start Time

- Game starts at **6:00 AM** (360 minutes)
- This gives players a full day to play on fresh start

## 9.3 UI Display

- Show time as `[06:00]` in top-right of game view
- Update every tick
- Paused state: `[06:00⏸]` or similar indicator

## 9.4 Future Considerations

- Sleep action (advance to 6:00 next day)
- Time-limited events (shop closes at 17:00)
- Seasonal time (longer days in summer, etc.)

--------------------------------------------------------------------
# 10. Telemetry

| Metric | Purpose |
|--------|---------|
| `heartbeat.tick_count` | Total ticks since game start |
| `heartbeat.pause_duration_ms` | Cumulative time spent paused |
| `heartbeat.drift_ms` | Difference between expected vs actual game time |
| `heartbeat.lag_spikes` | Count of ticks exceeding 200ms processing time |
| `heartbeat.day_transitions` | Number of day boundaries crossed |

All metrics reset on new game; persist across load for debugging.

--------------------------------------------------------------------
# 11. Decisions (Resolved)

1. **Offline time advancement**: **Disabled**. Time does not advance while the game is closed.
2. **Time speed controls**: **Not needed**. Keep fixed rate at 1 real second = 5 in-game minutes.
3. **Seasonal day-length variation**: **No**. Day length remains constant across seasons.
4. **NPC schedule system**: **No additional scheduling system required** for this scope.
5. **Multiplayer time sync**: **Not needed** (single-player scope).

