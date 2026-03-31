# Tinydew Engineering Spec

## Status
Draft.

## Overview
Tinydew is a cozy farming and exploration game built with Rust. It provides both an interactive TUI mode and a CLI interface. This document serves as the engineering blueprint for building the tinydew codebase, referencing the detailed specs in `agents/`.

## Language & Toolchain
- **Language**: Rust (stable toolchain).
- **Build**: `cargo build` (default features for MCP/CLI), `cargo build --features interactive` for TUI + audio.
- **Lint**: `clippy` enforced in CI.
- **Test**: `cargo test -- --test-threads=1` (single-threaded due to shared game state).

## Architecture

### Core Modules

| Module | Responsibility | Reference Spec |
|--------|---------------|----------------|
| `state` | Single authoritative game state, save/load, auto-save | `single-state-no-session.spec.md` |
| `map` | Region map creation (Farm, EastPath, Square, SouthRiver), tile types, walkability | `farm.spec.md`, `east-path.spec.md`, `square-region.spec.md`, `south-river-region.spec.md` |
| `entity` | Player and guest entities, position, movement, collision | `entities-and-movement.spec.md` |
| `economy` | Inventory (seeds, produce, forage, fish), shop buy/sell, money | `economy-and-items.spec.md` |
| `farming` | Clear, plant, water, harvest action handlers | `clear.spec.md`, `plant.spec.md`, `water.spec.md`, `harvest.spec.md`, `grow.spec.md` |
| `fishing` | Fish action, river bubble lifecycle, fish types | `fishing.spec.md` |
| `time` | Day/night cycle, 5-minute ticks, heartbeat | `day-night.spec.md`, `heartbeat.spec.md` |
| `sleep` | Sleep command, day transition, wake-up flow | `sleep-cycle.spec.md` |
| `weather` | Deterministic daily weather roll | `random-weather.spec.md` |
| `spawn` | Nightly random flower/mushroom spawn on valid tiles | `spawns.spec.md` |
| `festival` | Seasonal festival events (Spring Day 28 Butterfly Festival) | `seasonal-festival.spec.md` |
| `piano` | Audio playback, note mapping, guest piano interaction | `guest-piano-play.spec.md`, `piano-keyboard.spec.md`, `piano-samples.spec.md`, `farm-piano.spec.md` |
| `block_key` | One-row keyboard sound instrument (Q-P keys) | `block-key-sound.spec.md` |
| `ui` | Interactive TUI rendering, MCP print snapshot | `ui.spec.md` |
| `cli` | Command-line interface (`status`, `do` actions) | `cli.spec.md` |

### Tile System
- Enum `TileType` with variants: Grass, Tree, House, Gate, Fountain, Piano, River, RiverBubble, Crop, Mushroom, Flower, Wonder, etc.
- Each variant implements `is_walkable()` and `emoji()`.
- Protected tiles (House, Piano) excluded from random spawn logic.

### Region Maps
- **Farm**: Primary farming zone with tree boundaries, house tile, piano at `(4,2)`, east-path gate.
- **EastPath**: Connector corridor between Farm, Square, and SouthRiver with mushroom spawn area.
- **Square**: 9x5 with center fountain at `(4,2)`, no farming allowed.
- **SouthRiver**: 13x4 with river tiles for fishing, gate at top row.

### State Management
- One global game state instance — no per-session isolation.
- Persistent save/load on startup.
- Auto-save after command batches and day transitions.
- State fields: day, time, weather, player position, guest position, inventory, money, map tile states, crop growth data.

### Movement & Collision
- Directional movement: up/down/left/right.
- Blocked by: out-of-bounds, non-walkable tiles, same-region entity occupancy.
- Region transitions via gate tiles: Farm <-> EastPath, EastPath <-> Square, EastPath <-> SouthRiver.
- Mature crop tiles are non-walkable until harvested.

### Farming Loop
1. **Clear** — convert clearable tile to soil.
2. **Plant** — consume one generic seed, randomly roll crop type (Carrot/Strawberry/Cauliflower/Flower).
3. **Water** — set crop watered state; rainy weather auto-waters during day transition.
4. **Grow** — on day transition, watered crops increment `days_grown`; maturity based on type thresholds.
5. **Harvest** — collect produce from mature crops; tile reverts to ground.

### Day/Night & Sleep
- World time advances in 5-minute ticks per action.
- Sleep command advances to next morning 06:00, triggers day-start processing: weather roll, crop growth, river bubble reset, random spawns, festival checks.
- Wake-up position fixed at Farm `(3,3)`.
- Day 1 forced Sunny; Spring Day 28 forced Sunny (Butterfly Festival).

### Audio System (Feature-Gated)
- Gated behind `interactive` feature flag.
- Dependency: `rodio` crate (optional).
- Piano: 21 keys across 3 octaves (C3–B5), pitch-shifted from 9 Salamander Grand Piano FLAC samples stored in `./files/`.
- Dedicated audio thread with mpsc channel; max 4 concurrent sinks.
- Block key sound: 10-key sine wave instrument (Q–P), 500ms tones.
- Audio init failures silently ignored (best-effort).

### CLI Interface
- `tinydew status` — show current game state.
- `tinydew do <action> [args]` — execute actions: move, water, clear, plant, harvest, buy, sell, fish.
- Global flags: `-h`/`--help`, `-V`/`--version`.

### UI Rendering
- **Interactive TUI**: header (`tinydew day <day> <weather> <time>`), emoji tile map, player/guest markers, bottom message, compact controls line.
- **MCP print**: same header, map block, inventory lines (non-empty only), money line, bottom message.

## Build & CI
- See `ci.spec.md` for GitHub Actions workflows.
- CI workflow: checkout with submodules, stable Rust + clippy, `cargo build`, `cargo test -- --test-threads=1`.
- UI workflow: `cargo test initial_farm_ui -- --nocapture` smoke test.
- Interactive build: `cargo build --release --features interactive`.

## Dependencies
- `rodio` (optional, `interactive` feature) — audio playback.
- Salamander Grand Piano v3 samples (CC-BY, Alexander Holm) — stored in `./files/`.
- Standard Rust ecosystem: `serde` for state serialization, TUI libraries for interactive mode.

## Related Specs
All detailed specs are in the `agents/` directory. Each module above references its corresponding spec for full behavioral requirements.
