# Seeds Spec

## Status
Implemented.

## Model
- Seeds are tracked in inventory by crop type.

## Behavior
- Seeds are acquired via shop `buy` flow.
- Planting consumes one seed per planted crop tile.
- Seed counts are included in MCP state/stats snapshots.
- Seed inventory persists through save/load.
