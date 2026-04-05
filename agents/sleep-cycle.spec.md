# Sleep Cycle Spec

## Status
Implemented.

## Scope
Consolidates sleep, auto-sleep, and day-transition behavior.

## Behavior
- Sleep alert/income/auto-sleep forced flow is disabled.
- `tinydew do sleep` is supported and wakes to next morning checkpoint at `06:00`.
- Sleep wake-up position is fixed at Farm `(3,3)`, and both world/player location fields are synced to `Farm`.
- After midnight, gameplay can continue normally; bottom text suggests sleeping until `sleep` is called.
- Day-start processing includes:
  - weather roll (with festival override),
  - crop growth progression and watered-state reset,
  - river bubble reset,
  - random spawn steps,
  - seasonal wonder/festival checks.
