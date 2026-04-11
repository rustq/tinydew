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

## CI Cases

### Case 1: After midnight, day increments but day-start does not fire
- Player actions advance time past 00:00 (e.g. 23:55 → 00:00).
- Day counter increments (day 1 → day 2).
- Day-start processing (weather, crop growth, spawns, soil revert, festivals) does **not** run yet.

### Case 2: After 06:00, day-start fires automatically
- Player continues acting and time reaches 06:00.
- Day-start processing fires: weather rolls, crops grow, river bubbles reset, spawns occur, soil reverts, festival checks run.
- Day-start only fires once — further actions on the same day do not re-trigger it.

### Case 3: Harvest and water then sleep at 10:05 advances to next day
- After day-start fired at 06:00, player harvests and waters crops.
- Player sleeps at 10:05.
- Day counter increments (day 2 → day 3), time resets to 06:00, player moves to Farm (3,3).
- Day-start processing fires again for the new day: next weather rolls, crops grow, spawns occur, soil reverts, festival checks run.
