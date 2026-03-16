# MVP Task 06: East Path Foraging

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase F)

## Goal
Implement East Path forage gameplay and area-specific action rules.

## Todo
- [x] Spawn 0–2 mushrooms each morning on valid East Path grass tiles.
- [x] Enforce spawn exclusions (boundary, player tile).
- [x] Harvest rule: 🍄 -> 🌿 and add forage item.
- [x] Disable farming actions in East Path (clear/plant/water).
- [x] Show explicit feedback when blocked actions are attempted.

## Acceptance
- [x] Forage items spawn and can be harvested consistently.
- [x] Farm-only actions are blocked in East Path.

## Validation
- Spawn logic implemented in `state.rs:start_new_day()` with `spawn_east_path_mushrooms()`
- Mushroom harvest handled in `state.rs:harvest_action()` converting 🍄 -> 🌿 and adding to inventory
- Farming actions already blocked with explicit messages in `clear_action`, `plant_action`, `water_action`
- Build: cargo build passes
- Test: cargo test passes (1 test)
