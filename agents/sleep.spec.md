# Sleep and Day Transition Spec

## Status
Implemented.

## Behavior
- Interactive sleep flows through home menu states.
- MCP direct `sleep` command is disabled.
- Auto-sleep/day-advance handles reset to morning.
- Day transition performs weather roll, crop progression, bubble reset, spawns, and festival wonder checks.
