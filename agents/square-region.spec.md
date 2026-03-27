# Square Region Spec

## Status
Implemented.

## Map
- Dimensions: 9 x 5.
- Boundary trees block edges.
- Center fountain (`⛲`) at `(4,2)` is non-walkable.
- Piano (`🎹`) at Farm `(4,2)` is a permanent non-walkable fixture (see `north-square-piano.spec.md`).
- Bottom center gate connects to EastPath top-center gate.

## Behavior
- Supports player/guest movement with region-aware collision.
- Square forbids farm actions like clear/plant.
- Tile `(4,3)` acts as the piano play zone for the guest (see `guest-piano-play.spec.md`).
- Transition rules connect EastPath <-> Square for both player and guest.
- Entering Square from EastPath spawns at `(4,3)`.
- Returning from Square to EastPath spawns at `(5,1)`.
- Rendering works in interactive mode and MCP `print`/`getMap` outputs.
