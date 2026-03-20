# Player Entity Spec

## Status
Implemented.

## Behavior
- Player tracks `x/y/location/direction`.
- Player movement obeys walkability + transition rules.
- Player actions (farm/fish/trade) mutate game state.
- Player render marker appears only on active location map in MCP snapshot.
