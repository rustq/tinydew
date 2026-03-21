# Entities and Movement Spec

## Status
Implemented.

## Entities
- Player entity with `x/y/location/direction`.
- Guest entity with independent `x/y/location` and interactive control mode.

## Movement
- Directional movement uses up/down/left/right.
- Blocked by out-of-bounds, non-walkable tiles, and same-region occupancy.
- Mature crop tiles are non-walkable until harvested.
- Successful moves advance time per runtime rules.

## Guest Control
- Interactive mode enables guest control path.
- Space triggers guest greeting text.
- Guest obeys non-walkable tiles (including Fountain and Wonder).

## Transitions
- Farm <-> EastPath
- EastPath <-> Square
- EastPath <-> SouthRiver
- Transitions update location, coordinates, and contextual message for player/guest handlers.
