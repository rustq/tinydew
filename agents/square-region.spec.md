# Square Region Spec

## Status
Implemented.

## Map
- Dimensions: 9 x 5.
- Boundary trees block edges.
- Center fountain (`⛲`) is non-walkable.
- Bottom center gate connects to EastPath top-center gate.

## Behavior
- Supports player/guest movement with region-aware collision.
- Square forbids farm actions like clear/plant.
- Transition rules connect EastPath <-> Square for both player and guest.
- Rendering works in interactive mode and MCP `print`/`getMap` outputs.
