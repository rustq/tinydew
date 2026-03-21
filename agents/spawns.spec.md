# Spawns Spec

## Status
Implemented.

## Scope
Consolidates random crop/flower/mushroom-style world spawning rules.

## Behavior
- Daily spawn flow uses deterministic day/seed progression.
- Flower/decorative spawn picks valid empty grass tiles on allowed maps only.
- Mushroom spawn occurs on allowed maps and only on empty grass tiles; they remain non-walkable until collected/resolved.
- Mushroom visual/emoji in specs is 🍄.
- Protected tiles (house/wakeup-sensitive and similar guard tiles) are excluded from random placement.
- Square supports decorative spawn participation while preserving movement/collision constraints.
