# Entities and Movement Spec

## Status
Implemented.

## Entities
- Player entity with `x/y/location/direction`.

## Movement
- Directional movement uses up/down/left/right.
- Blocked by out-of-bounds, non-walkable tiles, and same-region occupancy.
- Mature crop tiles are non-walkable until harvested.
- If movement is blocked by a mature crop tile, movement feedback suggests harvesting first.
- Successful moves advance time per runtime rules.

## Transitions
- Farm <-> EastPath (return from EastPath to Farm spawns at `(6,5)`)
- EastPath <-> Square (entering Square spawns at `(4,3)`; returning from Square to EastPath spawns at `(5,1)`)
- EastPath <-> SouthRiver (return from SouthRiver spawns at EastPath `(2,2)`)
- Transitions update location, coordinates, and contextual message for the player.
