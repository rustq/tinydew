# Spawns Spec

## Status
Implemented.

## Scope
Nightly **random flower** and **random mushroom** spawns on the world map. Crops are **not** spawned this way — they come only from **planting** (see `plant.spec.md` / `grow.spec.md`).

## Behavior
- Daily spawn flow uses deterministic day/seed progression.
- **Flower**: spawn picks a valid empty grass tile in any region. Spawns already mature (non-walkable, harvestable).
- **Mushroom**: spawn occurs in any region on empty grass tiles. Spawns already mature (non-walkable, harvestable).
- Per region, nightly random spawn allows max one active flower AND max one active mushroom (one of each type per region). Pre-placed flowers/mushrooms (e.g. EastPath mushroom at (9,2)) count toward this limit.
- Mushroom visual/emoji in specs is 🍄.
- Protected tiles (house/wakeup-sensitive and similar guard tiles) are excluded from random placement.
- Square supports decorative spawn participation while preserving movement/collision constraints.
