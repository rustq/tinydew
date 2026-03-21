# Spawns Spec

## Status
Implemented.

## Scope
Consolidates random crop/flower/mushroom-style world spawning rules.

## Behavior
- Daily spawn flow uses deterministic day/seed progression.
- Flower/decorative spawn picks valid grass tiles on allowed maps.
- Mushroom spawn occurs on allowed maps with non-walkable tile behavior until collected/resolved.
- Mushroom visual/emoji in specs is 🍄.
- Protected tiles (house/wakeup-sensitive and similar guard tiles) are excluded from random placement.
- Square supports decorative spawn participation while preserving movement/collision constraints.
