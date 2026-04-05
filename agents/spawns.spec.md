# Spawns Spec

## Status
Implemented.

## Scope
Nightly **random flower** and **random mushroom** spawns on the world map. Crops are **not** spawned this way — they come only from **planting** (see `plant.spec.md` / `grow.spec.md`).

## Behavior
- Daily spawn flow uses deterministic day/seed progression.
- **Flower**: spawn picks a valid empty grass tile on allowed maps only.
- **Mushroom**: spawn occurs on allowed maps and only on empty grass tiles; mushrooms remain non-walkable until collected/resolved.
- Per region, nightly random spawn executes only when that region currently has no flower/mushroom present (max one active flower-or-mushroom blocker per region).
- Mushroom visual/emoji in specs is 🍄.
- Protected tiles (house/wakeup-sensitive and similar guard tiles) are excluded from random placement.
- Square supports decorative spawn participation while preserving movement/collision constraints.
