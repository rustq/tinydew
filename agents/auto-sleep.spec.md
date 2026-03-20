# Auto Sleep Spec

## Status
Implemented.

## Behavior
- Sleep command is disabled in MCP command surface.
- Auto-sleep flow handles day transition and morning reset.
- Day transition processes crops/weather/spawns and resets timing state.
- Post-sleep state returns to morning playable state.
