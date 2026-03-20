# Money / Economy Spec

## Status
Implemented.

## Behavior
- Game starts with baseline money.
- Buying items decreases money by item price × quantity.
- Selling produce/forage/fish increases money by sell value × quantity.
- Daily income summary fields are tracked in state for sleep/day reporting.
- Money is exposed in interactive header/snapshots and MCP state/stats.
