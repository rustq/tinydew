# Fishing Spec

## Status
Implemented.

## Behavior
- Fishing command: `fishing[:dir]` in MCP.
- Default fishing supports adjacent auto-targeting near river.
- Fishing advances time and can yield common/rare fish or no bite.
- River bubble tile lifecycle is supported and reset by sleep/day transition.
- Fish inventory persists via save/load.
- Fish can be sold (`sell:fish`, `sell:rare`) with value tracking.
