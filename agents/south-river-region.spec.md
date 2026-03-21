# South River Region Spec

## Status
Implemented.

## Canonical Layout
- Region dimensions: 13 x 4.
- Gate at top row column 2.
- Transition pairing with EastPath is implemented.

## Behavior
- River tiles are fishable and support bubble state transitions.
- Movement/collision respects boundaries and river non-walkable rules.
- Enter/exit transitions update location and entity coordinates.
- Returning from SouthRiver to EastPath spawns at `(2,2)`.
