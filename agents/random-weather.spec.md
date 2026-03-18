# Random Weather System Specification (Spring)

## Overview
Define deterministic daily random weather selection for **Spring** and apply weather effects to farm crops.

Spring daily weather probabilities:
- **Sunny** `☀️`: **80%**
- **Cloudy** `⛅`: **13%**
- **Rainy** `🌧️`: **7%**

If weather is **Rainy**, all eligible planted crops are automatically watered by rain.

Weather icon is shown in the **top UI during daytime**.

---

## 1) Goals

1. Add season-aware random daily weather for Spring.
2. Ensure probabilities match exactly 80/13/7.
3. Apply automatic crop watering effect on rainy days.
4. Display daytime weather icon in top game UI.
5. Keep behavior deterministic for saves/tests (seeded RNG).

---

## 2) Non-Goals

1. No per-tile rain intensity system.
2. No thunderstorm/snow/fog weather types in this phase.
3. No weather changes within the same day (one weather per day).
4. No balancing changes to crop growth rates beyond watering effect.

---

## 3) Weather Model

## 3.1 Spring Distribution

For each new day in Spring, sample one weather state using weighted probabilities:

- `Sunny` weight: 80
- `Cloudy` weight: 13
- `Rainy` weight: 7
- Total weight: 100

Equivalent threshold mapping for random integer `r in [0, 99]`:

- `0..=79` -> `Sunny ☀️`
- `80..=92` -> `Cloudy ⛅`
- `93..=99` -> `Rainy 🌧️`

## 3.2 Day Scope

- Weather is selected **once at day start**.
- Weather remains fixed until next day starts.

---

## 4) Weather Selection Timing

Weather selection must occur at a single canonical day-transition point:

1. New game start (Day 1 weather initialization)
2. Sleep/day rollover (manual sleep or auto sleep path)
3. Any direct day-advance API/path

Do not re-roll weather on map changes, menu open/close, or regular time ticks.

---

## 5) Rain Watering Effect

## 5.1 Rule

If today’s weather is `Rainy 🌧️`, all eligible planted crops are marked watered automatically.

## 5.2 Eligibility

A crop is eligible if:
- It is planted on a valid farm tile
- It can receive water in the current growth state
- It has not already been harvested/removed

## 5.3 Application Timing

Recommended: apply rain watering once at day start immediately after weather selection.

Alternative (acceptable if consistent): apply on first daytime update of that day.

## 5.4 Idempotency

Rain watering must be idempotent per day:
- Re-running same day state logic should not double-apply side effects.

---

## 6) UI Rules

## 6.1 Top UI Daytime Icon

During **daytime**, top UI displays today’s weather icon:
- Sunny -> `☀️`
- Cloudy -> `⛅`
- Rainy -> `🌧️`

## 6.2 Night Interaction

Night icon behavior follows existing day-night spec rules.
If night currently forces moon icon (`🌙`), keep that behavior unchanged.

Implication:
- Day: show weather icon (`☀️/⛅/🌧️`)
- Night: existing night icon rule (e.g., `🌙`)

---

## 7) Data Model

## 7.1 Persisted Fields

Store at least:
- `current_weather` (enum/string)
- `weather_day` or equivalent marker to bind weather to a specific day number

Optional but recommended:
- RNG seed/state for deterministic replay/testing

## 7.2 Enum (illustrative)

```rust
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}
```

---

## 8) Save/Load Behavior

1. Loading mid-day must preserve that day’s weather exactly.
2. Loading a rainy day should preserve already-applied watering state.
3. Next day weather should re-roll once at next day transition.
4. No migration required if backward-compatible defaults are defined.

---

## 9) MCP / API Behavior

If MCP exposes state/snapshots:

1. Include current weather in structured state.
2. Ensure weather remains stable within the same day.
3. On rainy day, crop watered state should be observable in returned state/map.
4. Day-transition events should include newly selected weather.

---

## 10) Determinism & RNG

1. Use a seeded RNG path where possible.
2. Given same seed and same sequence of day transitions, weather sequence must match.
3. Avoid hidden extra RNG draws that can desync expected distribution in tests.

---

## 11) Edge Cases

1. **Day skip/jump:** If day advances by >1 in a single operation, select weather once per resulting day transition (or document exact chosen behavior and keep consistent).
2. **Save at rollover boundary:** Must not duplicate re-roll on load.
3. **Rain + manually watered crops:** No adverse effect; watered remains true.
4. **No crops planted on rainy day:** System still valid; rain effect is a no-op.

---

## 12) Testing Plan

## 12.1 Unit Tests

1. Threshold mapping correctness (`0..79`, `80..92`, `93..99`).
2. Weather selected once per day transition.
3. Rainy day sets watered=true for eligible crops.
4. Rain application is idempotent within same day.

## 12.2 Statistical/Property Tests

Over large sample (e.g., 100,000 Spring days), observed distribution approximates:
- Sunny ~80%
- Cloudy ~13%
- Rainy ~7%

Allow small tolerance band (e.g., ±0.5% to ±1.0%).

## 12.3 Integration Tests

1. Day rollover selects and displays new weather icon in daytime UI.
2. Night UI still follows moon/night icon rule.
3. Rainy day auto-waters crops and growth logic recognizes watered state.
4. Save/load preserves current day weather and watered results.

## 12.4 MCP Tests

1. `getState` reports expected weather for current day.
2. Rainy day state reflects watered crops after rollover.

---

## 13) Rollout Plan

1. Add `Weather` enum and persisted weather state.
2. Implement Spring weighted selection at day-transition point.
3. Add rainy-day auto-watering pass.
4. Wire top UI daytime icon rendering to current weather.
5. Add tests (unit/statistical/integration/MCP).
6. Validate no regression with day-night icon priority.

---

## 14) Acceptance Criteria

Accepted criteria:

2. Weather is selected once per day and stays constant for that day.
3. On rainy days, eligible crops are auto-watered.
4. During daytime, top UI shows correct weather icon.
5. Existing night icon behavior remains correct and not regressed.
6. Save/load and MCP state remain consistent with selected weather and rain effects.

Criterion 1 is **not accepted** at this time.
