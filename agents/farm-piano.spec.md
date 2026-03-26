# Farm Piano Spec

## Status
Implemented.

## Context
The Farm region receives a permanent decorative piano object. The piano is a non-walkable fixture placed at a specific tile.

## Placement
- Piano tile (`🎹`) placed at Farm `(4,2)`.
- Replaces the default Grass tile at that position.
- Piano is always present on the map (not seasonal or spawned).
- The tile sits on row 2, two tiles right of center.

## Map Reference (Farm, row 2 after placement)
```
🌳 🌿 🌿 🌿 🌿 🌿 🌿 🌿 🌳
```

## Walkability
- Piano is non-walkable for both player and guest.
- Movement, pathfinding, and collision must respect the piano as a blocking tile.
- Random spawn logic must treat `(4,2)` as a protected tile (no flower/mushroom spawn on it).

## Interaction
- Attempting to walk onto the piano tile shows:
  `A beautiful old piano. It hums quietly on the farm.`
- Guest can play the piano by standing at `(4,3)` directly below the piano tile (see `guest-piano-play.spec.md`).

## Implementation Notes
- Requires new `TileType::Piano` variant in the tile enum.
- `is_walkable()` must return `false` for `TileType::Piano`.
- `emoji()` must return `"🎹"` for `TileType::Piano`.
- `create_farm_map()` sets `TileType::Piano` at `farm_map[2][4]`.
- Protected tile list in spawn logic must include Piano (or the `(4,2)` coordinate in Farm).

## Related Specs
- `guest-piano-play.spec.md` — Piano playing mechanics.
- `entities-and-movement.spec.md` — Guest movement and non-walkable tile rules.
- `farm-region.spec.md` — Farm map layout.
