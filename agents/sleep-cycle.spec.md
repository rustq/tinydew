# Sleep Cycle Spec

## Status
Implemented.

## Scope
Consolidates sleep, auto-sleep, and day-transition behavior.

## Behavior
- Sleep alert/income/auto-sleep forced flow is disabled.
- MCP direct `sleep` command is disabled.
- After midnight, gameplay continues normally while bottom text suggests sleeping.
- Day-start processing includes:
  - weather roll (with festival override),
  - crop growth progression and watered-state reset,
  - river bubble reset,
  - random spawn steps,
  - seasonal wonder/festival checks.
