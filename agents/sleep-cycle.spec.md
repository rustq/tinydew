# Sleep Cycle Spec

## Status
Implemented.

## Scope
Consolidates sleep, auto-sleep, and day-transition behavior.

## Behavior
- Interactive sleep uses home/menu flow and transitions into income/day-advance flow.
- MCP direct `sleep` command is disabled.
- Auto-sleep handles overnight transition and reset to morning playable state.
- Day-start processing includes:
  - weather roll (with festival override),
  - crop growth progression and watered-state reset,
  - river bubble reset,
  - random spawn steps,
  - seasonal wonder/festival checks.
