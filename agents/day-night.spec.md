# Day–Night System Specification

## Overview
Define a deterministic day–night cycle for Shelldew based on in-game clock time.

- **Day:** `06:00` (inclusive) to `18:00` (exclusive)
- **Night:** `18:00` (inclusive) to next day `06:00` (exclusive)
- During **night**, weather icon is forced to `🌙`.

---

## 1) Goals

1. Add clear visual time-of-day state from existing in-game time.
2. Make transitions predictable and rule-based.
3. Keep implementation lightweight and fully deterministic.

---

## 2) Non-Goals

1. No gameplay stat changes (crop growth, movement, prices, etc.) in this phase.
2. No dynamic sky gradients/lighting system in this phase.
3. No random night weather icon variants in this phase.

---

## 3) Time-of-Day Rules

Given in-game time as `(hour, minute)` in 24h format:

- **is_day = true** when `06 <= hour < 18`
- **is_night = true** otherwise

Boundary behavior:
- `05:59` → Night
- `06:00` → Day
- `17:59` → Day
- `18:00` → Night

---

## 4) Weather Icon Rules

### 4.1 Display Priority

1. If `is_night == true`, render weather icon as **`🌙`**.
2. Else render existing daytime weather icon logic unchanged.

### 4.2 Scope

This applies to all places where the weather icon is displayed in game UI and MCP-readable text snapshots (if icon is surfaced there).

---

## 5) Data & API Surface

## 5.1 Derived State

No new persisted field required. `is_day` / `is_night` are derived from current clock time.

## 5.2 Optional Helper API

Introduce helper functions (names illustrative):

- `fn is_day(hour: u8, minute: u8) -> bool`
- `fn is_night(hour: u8, minute: u8) -> bool`
- `fn get_display_weather_icon(base_weather_icon: &str, hour: u8, minute: u8) -> &str`

---

## 6) Integration Points

1. **UI header/time row:** weather icon rendering should route through day–night icon selection.
2. **Any map/status renderer:** use the same helper to avoid duplicated logic.
3. **MCP/print snapshots (if icon shown):** reuse same helper for consistency.

---

## 7) Save/Load Behavior

No migration needed.

- Save already stores game time.
- After load, day/night state and icon are re-derived from loaded time.

---

## 8) Determinism & Edge Cases

1. System must be fully deterministic from clock time only.
2. Transition occurs exactly at minute boundary (`06:00`, `18:00`).
3. No flicker at boundaries: single source of truth helper should be used everywhere.

---

## 9) Testing Plan

## 9.1 Unit Tests

1. `05:59` returns `is_night = true`, icon `🌙`
2. `06:00` returns `is_day = true`, icon is daytime weather icon
3. `17:59` returns `is_day = true`, icon is daytime weather icon
4. `18:00` returns `is_night = true`, icon `🌙`

## 9.2 Integration Tests

1. Start near `05:55`, advance to `06:00`, verify icon switches from `🌙` to daytime icon.
2. Start near `17:55`, advance to `18:00`, verify icon switches to `🌙`.
3. Save at night (`e.g. 22:10`) and reload, verify icon remains `🌙`.
4. Save at day (`e.g. 09:30`) and reload, verify daytime icon logic remains active.

---

## 10) Rollout Plan

1. Add day/night helper functions.
2. Route weather icon rendering through helper.
3. Add unit and integration coverage.
4. Verify snapshots/UI consistency.

---

## 11) Acceptance Criteria

Feature is complete when:

1. Day period is exactly `06:00–17:59`.
2. Night period is exactly `18:00–05:59`.
3. Weather icon is always `🌙` at night.
4. Daytime weather icon behavior remains unchanged.
5. Behavior is consistent after save/load.
