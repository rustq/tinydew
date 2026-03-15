# MVP Task 01: World & Movement Foundation

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase A)

## Goal
Implement Farm + East Path maps and arrow-key movement with collision + area transitions.

## Todo
- [x] Create Farm map using MVP 8x8 layout and boundary rules (no Player tile in map; player from state).
- [x] Create East Path dead-end map with return transition tile (no Player tile in map).
- [x] Implement passability rules (tree blocked, grass/soil/crops walkable).
- [x] Implement Arrow-key movement only for MVP mode.
- [x] Track facing direction for “tile in front” interactions.
- [x] Add Farm ↔ East Path transition handling (player position set on transition).
- [x] Terminal display: CRLF line endings; path tiles may render as 🌿; house 🏠; player drawn only from (player_x, player_y).

## Acceptance
- [x] Player cannot move through boundary trees.
- [x] Player can traverse valid walkable tiles.
- [x] Transition to East Path works from Farm exit tile.
- [x] Return transition works from East Path back tile.
- [x] Map renders as aligned grid; single player (no Player tile in map data).
