# Movement Spec

## Status
Implemented.

## Rules
- Movement uses `up/down/left/right` directions.
- Out-of-bounds movement is blocked.
- Non-walkable tiles are blocked.
- Occupied tile by the other entity in same region is blocked.
- Successful movement advances world time (unless guest-time-frozen interactive context).
- Movement onto transition tiles triggers region transition handlers.

## Messaging
- Blocked: `Cannot move there!`
- Occupied: `Tile occupied.`
