# Tinydew Engineering Spec

## Status
Draft.

## Overview
Tinydew is a cozy farming and exploration game built with Rust. It provides a **CLI** interface only (`tinydew status`, `tinydew do …`) — **no interactive/TUI mode**. **No realtime audio dependency** (no `rodio` or equivalent required).

## Language & Toolchain
- **Language**: Rust (stable toolchain).
- **Build**: `cargo build` compiles the CLI binary. Release: `cargo build --release`.
- **Lint**: `clippy` enforced in CI.
- **Test**: `cargo test -- --test-threads=1` (single-threaded due to shared game state).

## Architecture

### Core Modules

| Module | Responsibility | Reference Spec |
|--------|---------------|----------------|
| `state` | Single authoritative game state; SQLite persistence, load/save, auto-save | `single-state-no-session.spec.md` |
| `map` | Region map creation (Farm, EastPath, Square, SouthRiver), tile types, walkability | `initial-map.spec.md`, `farm.spec.md`, `east-path.spec.md`, `square-region.spec.md`, `south-river-region.spec.md`, `tree.spec.md`, `game-emoji-map.spec.md` |
| `entity` | Player entity, position, movement, collision | `entities-and-movement.spec.md` |
| `economy` | Inventory (seeds, produce, forage, fish), shop buy/sell, money | `economy-and-items.spec.md` |
| `farming` | Clear, plant, water, harvest action handlers | `clear.spec.md`, `plant.spec.md`, `water.spec.md`, `harvest.spec.md`, `grow.spec.md` |
| `fishing` | Fish action, river bubble lifecycle, fish types | `fishing.spec.md` |
| `time` | Day/night cycle, 5-minute ticks, heartbeat | `day-night.spec.md`, `heartbeat.spec.md` |
| `sleep` | Sleep command, day transition, wake-up flow | `sleep-cycle.spec.md` |
| `weather` | Deterministic daily weather roll | `random-weather.spec.md` |
| `spawn` | Nightly random flower/mushroom spawn on valid tiles | `spawns.spec.md` |
| `festival` | Seasonal festival events (Spring Day 28 Butterfly Festival) | `seasonal-festival.spec.md` |
| `ui` | Plain-text game view for `tinydew status` output | `ui.spec.md` |
| `cli` | Command-line interface (`status`, `do` actions) | `cli.spec.md` |

### Tile System
- Enum `TileType`: **canonical emoji and tile names** are in `game-emoji-map.spec.md`. **Coordinates and region graphs** are in `initial-map.spec.md` plus each region spec. Edges use `Boundary` (non-walkable); region links use path/transition tile variants (`PathEast`, `PathFarm`, etc.), not a separate abstract “Gate” type, unless implementation chooses an alias.
- Each variant implements `is_walkable()` and `emoji()`.
- Protected tiles (e.g. House) excluded from random spawn logic.

### Region Maps
- **Farm**: Primary farming zone with tree boundaries, house tile, east-path gate.
- **EastPath**: Connector corridor between Farm, Square, and SouthRiver with mushroom spawn area.
- **Square**: 9x5 with center fountain at `(4,2)`, no farming allowed.
- **SouthRiver**: 13x4 with river tiles for fishing, gate at top row.

### State Management
- One global game state instance — no per-session isolation.
- **Persistence**: SQLite database file (see `single-state-no-session.spec.md` for path, schema versioning, WAL, and transactions).
- Load from SQLite on startup; auto-save to SQLite after command batches and day transitions.
- State fields: day, time, weather, player position, inventory, money, map tile states, crop growth data.

### Movement & Collision
- Directional movement: up/down/left/right.
- Blocked by: out-of-bounds, non-walkable tiles, same-region entity occupancy.
- Region transitions via gate tiles: Farm <-> EastPath, EastPath <-> Square, EastPath <-> SouthRiver.
- Mature crop tiles are non-walkable until harvested.

### Farming Loop
1. **Clear** — convert clearable tile to soil.
2. **Plant** — consume one generic seed, randomly roll crop type (Carrot/Strawberry/Cauliflower).
3. **Water** — set crop watered state; rainy weather auto-waters during day transition.
4. **Grow** — on day transition, watered crops increment `days_grown`; all crops mature in 1 day if watered.
5. **Harvest** — collect produce from mature crops; crop tile reverts to immature replant. Mushroom/flower harvest returns tile to soil.

### Day/Night & Sleep
- World time advances in 5-minute ticks per action (exception: fishing costs 1 hour).
- Sleep command advances to next morning 06:00, triggers day-start processing: weather roll, crop growth, river bubble reset, random spawns, festival checks.
- Wake-up position fixed at Farm `(3,3)`.
- Day 1 forced Sunny; Spring Day 28 forced Sunny (Butterfly Festival).

### CLI Interface
- `tinydew status` — show current game state.
- `tinydew do <action> [args]` — execute actions: move, water, clear, plant, harvest, buy, sell, fish, sleep (full list in `cli.spec.md`).
- Global flags: `-h`/`--help`, `-V`/`--version`.

### UI Rendering
- **CLI `status`** output includes: header (`tinydew day <day> <weather> <time>`), emoji tile map, inventory lines (non-empty only), money line, bottom message (see `ui.spec.md`).

## Build & CI
- See `ci.spec.md` for GitHub Actions workflows.
- CI workflow: checkout, stable Rust + clippy, `cargo build`, `cargo test -- --test-threads=1`.
- UI workflow: `cargo test initial_farm_ui -- --nocapture` smoke test.
- Release build: `cargo build --release`.

## Dependencies
- **`rusqlite`** (or equivalent) — SQLite access; prefer **`bundled`** sqlite for consistent CI and local builds unless documented otherwise.
- `serde` (optional) — still useful for JSON/TEXT column encoding inside SQLite or ancillary config; not a replacement for the DB file.
- No TUI libraries needed — pure CLI output.

## Related Specs
All detailed specs are in the `agents/` directory. Each module above references its corresponding spec for full behavioral requirements.
