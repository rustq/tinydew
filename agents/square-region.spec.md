# Square Region Spec

## Status
Implemented.

## Map
- Dimensions: 9 x 5.
- Boundary trees block edges.
- Center fountain (`⛲`) at `(4,2)` is non-walkable.
- Bottom center gate connects to EastPath top-center gate.

## Behavior
- Supports player/guest movement with region-aware collision.
- Square forbids farm actions like clear/plant.
- Transition rules connect EastPath <-> Square for both player and guest.
- Entering Square from EastPath spawns at `(4,3)`.
- Returning from Square to EastPath spawns at `(5,1)`.
- Rendering works in interactive mode and MCP `print`/`getMap` outputs.
