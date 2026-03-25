# North Square Piano Spec

## Status
Implemented.

## Context
The Square region (the northernmost plaza) receives a permanent decorative piano object. The piano is a non-walkable fixture placed at a specific tile.

## Placement
- Piano tile (`🎹`) placed at Square `(6,2)`.
- Replaces the default Grass tile at that position.
- Piano is always present on the map (not seasonal or spawned).
- The tile sits on row 2 (the fountain row), two tiles right of the fountain (`⛲` at `(4,2)`).

## Map Reference (Square, row 2 after placement)
```
🌳 🌿 🌿 🌿 ⛲ 🌿 🎹 🌿 🌳
```

## Walkability
- Piano is non-walkable for both player and guest.
- Movement, pathfinding, and collision must respect the piano as a blocking tile.
- Random spawn logic must treat `(6,2)` as a protected tile (no flower/mushroom spawn on it).

## Interaction
- Attempting to walk onto the piano tile shows:
  `A beautiful old piano. It hums quietly in the square.`

## Implementation Notes
- Requires new `TileType::Piano` variant in the tile enum.
- `is_walkable()` must return `false` for `TileType::Piano`.
- `emoji()` must return `"🎹"` for `TileType::Piano`.
- `create_square_map()` sets `TileType::Piano` at `square_map[2][6]`.
- Protected tile list in spawn logic must include Piano (or the `(6,2)` coordinate in Square).
