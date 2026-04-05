# Farm Region Spec

## Status
Implemented.

## Map
- Fixed-size farm map with tree boundaries.
- Contains house tile and east-path transition gate.
- Primary farming zone for clear/plant/water/harvest loops.

## Behavior
- Supports crops, random spawn flows, and daily progression.
- EastPath -> Farm return spawn is `(6,5)`.
- Home/sleep flow is anchored to farm context.
