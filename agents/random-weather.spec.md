# Random Weather Spec

## Status
Implemented with festival override.

## Behavior
- Weather rolls daily using deterministic seed-based logic.
- Distribution favors Sunny, then Cloudy, then Rainy.
- Day 1 is forced Sunny.
- Spring Day 28 (Butterfly Festival) is forced Sunny.
- Weather state is reflected in the TUI, `tinydew status`, and persisted game state.
