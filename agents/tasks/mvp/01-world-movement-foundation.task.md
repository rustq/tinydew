# MVP Task 01: World & Movement Foundation

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase A)

## Goal
Implement Farm + East Path maps and arrow-key movement with collision + area transitions.

## Todo
- [ ] Create Farm map using MVP 8x8 layout and boundary rules.
- [ ] Create East Path dead-end map with return transition tile.
- [ ] Implement passability rules (tree blocked, grass/soil/crops walkable).
- [ ] Implement Arrow-key movement only for MVP mode.
- [ ] Track facing direction for “tile in front” interactions.
- [ ] Add Farm ↔ East Path transition handling.

## Acceptance
- [ ] Player cannot move through boundary trees.
- [ ] Player can traverse valid walkable tiles.
- [ ] Transition to East Path works from Farm exit tile.
- [ ] Return transition works from East Path back tile.
