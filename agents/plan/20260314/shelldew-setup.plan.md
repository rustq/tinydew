# Shelldew Setup Plan

- **Date:** 2026-03-14
- **Owner:** Hephaestus Automation Cell
- **Source Spec:** `agents/shelldew.spec.md`

## Objectives

1. Stand up a cozy emoji-first CLI farming sim that satisfies the MVP listed in the spec.
2. Preserve responsiveness (direct WASD movement, one-hour day length) while keeping the codebase maintainable.
3. Deliver tooling and workflows (build, lint, test) so future agents can iterate confidently.

## Guiding Constraints

- No stamina or fog mechanics.
- Emoji tiles drive the presentation wherever feasible; ASCII fallbacks only for debugging.
- The starting farm must look messy, natural, and partially obstructed per the provided layout.
- Real-time loop targets ~2.5 seconds per in-game minute (≈1 hour per in-game day).

## Scope

### In Scope

- Map renderer + input loop and timekeeping foundation.
- Farming verbs: clearing, tilling, planting, watering, harvesting.
- Weather + four-season state machine influencing crops, visuals, and schedules.
- Fishing, cat interaction, town travel, shopping/selling, NPC dialogue.
- Save/load pipeline covering player, world, and relationship state.

### Out of Scope (for setup phase)

- Romance arcs, festivals, mines, combat, late-game crafting.
- Advanced fishing minigames beyond the basic cast/wait/result loop.
- Multiplayer or networked features.

## Workstreams & Tasks

### W1. Engine & World Loop

1. Establish Rust crates/modules for engine, rendering, and data.
2. Build real-time tick scheduler hitting the 2.5s minute cadence.
3. Implement WASD + arrow movement with collision against map tiles.
4. Author initial farm, river, town maps as emoji grids; load via data files for easy iteration.

### W2. Farming Systems

1. Represent terrain tiles (grass, weeds, soil, debris) and crop states (seed→mature) with emoji assets.
2. Implement clearing (hoe, axe, pickaxe) interactions with proper drop/inventory hooks.
3. Support till -> plant -> water loop with per-day growth resolution driven by time/weather/season.
4. Hook harvest results into inventory and shop sell actions.

### W3. Environment & Atmosphere

1. Build season manager (28-day cycles) updating palette, available crops, and fishing tables.
2. Implement daily weather roll (sunny/cloudy/rainy/stormy/snow) with effects (auto-watering, fishing modifiers, visuals).
3. Render top status bar (date, time, weather, money) and bottom help bar per spec.

### W4. Social, Town, Cat, and Commerce

1. Script NPC definitions (name, schedule, dialogue pools) and dialogue factors (season/time/weather/relationship).
2. Add town scenes (general store, saloon, square) with interaction hotspots (💬, 💰).
3. Implement shop inventory browsing, buy/sell flows, and currency tracking.
4. Place cat entity with daily interaction limit and flavorful responses.

### W5. Fishing System

1. Build water-adjacent interaction detection and fishing command flow.
2. Define catch tables influenced by season, weather, time, and location.
3. Emit results via emoji messages (`🎣 You caught …`) and inventory integration.

### W6. Persistence & Tooling

1. Design save-game schema covering player position, inventory, crops, cleared tiles, weather, cat/NPC state.
2. Implement autosave at sleep and load on boot.
3. Wire up `cargo fmt`, `cargo clippy`, and structured test suites (`cargo test`, integration runners) into CI scripts.
4. Document verification commands inside `agents/constitution.md` reference section.

## Milestones & Sequencing

| Milestone | Target | Contents |
| --- | --- | --- |
| M1 Foundations | Week 1 | Renderer, input, movement, timekeeper, static map render |
| M2 Farming Loop | Week 2 | Clearing/tilling/watering/planting/harvest, inventory basics |
| M3 Weather & Seasons | Week 3 | Season+weather systems affecting visuals and growth |
| M4 Fishing + Cat | Week 4 | Fishing loop, cat interactions, messaging polish |
| M5 Town & Social | Week 5 | Town scenes, NPC dialogue, shop buy/sell |
| M6 Persistence & Polish | Week 6 | Save/load, autosave, UI polish, verification pipeline |

Dependencies are primarily linear: M2 builds on M1, M3 requires farm loop, M4 uses season/weather data, M5 depends on movement/town scenes, and M6 needs all prior state tracked.

## Risks & Mitigations

- **Emoji rendering inconsistency across terminals** → standardize on Unicode 15 baseline and include fallback map visualizer for unsupported glyphs.
- **Real-time loop drift** → add diagnostics and delta capping; consider `winit`/`crossterm` timing helpers.
- **Complex save-state bugs** → implement serialization tests and versioned save schema.
- **NPC dialogue bloat** → start with structured templates keyed by (season, weather, time) and expand gradually.

## Acceptance & Verification

- Each milestone closes only after `cargo fmt`, `cargo clippy`, and relevant `cargo test` suites pass.
- Manual smoke checklist: load game, traverse farm→town, perform farming loop, trigger weather change, fish, pet cat, shop, and save/load.
- Plans, status updates, and blockers recorded under `agents/plan/` and linked in agent READMEs.
