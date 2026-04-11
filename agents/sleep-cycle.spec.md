# Sleep Cycle Spec

## Status
Implemented.

## Scope
Consolidates sleep, auto-sleep, and day-transition behavior.

## Behavior
- Sleep alert/income/auto-sleep forced flow is disabled.
- `tinydew do sleep` is supported and wakes to next morning checkpoint at `06:00`.
- Sleep wake-up position is fixed at Farm `(3,3)`, and both world/player location fields are synced to `Farm`.
- The day counter increments when time crosses midnight. Day-start processing triggers automatically once time reaches 06:00.
- Day-start processing runs in this order:
  1. weather roll (with festival override),
  2. crop growth check (watered crops grow), then watered-state reset,
  3. river bubble reset,
  4. random spawn steps (on empty grass tiles only),
  5. soil reverts to grass (after spawns, so soil tiles skip this night's spawn),
  6. seasonal wonder/festival checks.
