# Shelldew Features Summary

## 1) Core Gameplay

- **Tile-based movement** on farm and east-path maps.
- **Boundary collision** prevents moving outside map bounds.
- **Map transitions** between Farm and East Path via path tiles.
- **Time progression** advances with player actions.
- **Day progression** via sleep loop.

## 2) Farming System

- **Clear action** converts clearable grass/weeds into cleared soil (`🍃`).
- **Planting** supports directional targeting via MCP command forms.
- **Watering** supports directional targeting and tracks watered status.
- **Growth logic** advances crops based on day transitions and watering rules.
- **Maturity states**:
  - Seedling (`🌱`) while growing
  - Produce emoji when mature (e.g., `🥕`, `🍓`, `🥦`, `🌺`)
- **Harvesting** supports directional targeting and adds produce to inventory.

## 3) Crop & Movement Rules

- **Crop tiles are non-walkable** (seedling and mature crop states).
- **Mushroom tiles are non-walkable** (must harvest from adjacent target direction).
- **Initial farm includes a mature flower crop** at `(6,2)` (`🌺`, Rhubarb).

## 4) Tile Lifecycle Behavior

- Cleared tile is shown as **`🍃`**.
- **New-day cleanup rule**: if a cleared soil tile has no crop planted, it reverts to grass (`🌿`) next day.
- Planted crop tiles are preserved and not reverted by cleanup.

## 5) Economy & Inventory

- Buy/sell flow for seeds and produce.
- Seed inventory and produce inventory tracked independently.
- Money updates through market actions.
- Forage inventory supports items such as mushroom (`🍄`).

## 6) East Path Foraging

- East Path area supports forage interactions.
- Mushroom harvesting available through directional harvest targeting.

## 7) Sleep & Home Loop

- Sleep triggers day advancement and morning reset.
- Wake-up behavior restored to **home-front spawn on Farm**.
- Morning state remains playable after day transition.

## 8) MCP Runtime Features

Shelldew runs as MCP-first runtime and supports JSON-over-stdio commands.

### Session APIs

- `startSession`
- `endSession`
- session-bound gameplay state

### Action APIs

- `command`
- `commandBatch`

### Query APIs

- state/map/stats retrieval commands
- print-based text UI snapshots

### Directional Command Targeting

Supports directional forms for adjacent-tile actions, including:

- `clear:<up|down|left|right>`
- `plant:<crop>:<up|down|left|right>`
- `water:<up|down|left|right>`
- `harvest:<up|down|left|right>`

(Backward-compatible non-direction forms are also supported.)

## 9) Save/Load Persistence (Local)

- **Save command** persists full game state to local disk.
- **Load command** restores saved state.
- Default local path:
  - `/root/.local/share/shelldew/savegame.json` (environment-dependent local data directory in general).
- Handles missing/corrupt save data via structured errors.

## 10) UI / Snapshot Output

- `print` returns full textual game snapshot including:
  - day/time/location/money
  - player position
  - inventory summary
  - rendered map with emojis
  - latest message/event
- Player marker (`🧑`) is shown correctly in MCP snapshot output.

## 11) Quality Gates & CI

- Tests cover core gameplay, MCP handlers, command parsing, session lifecycle, and save/load behavior.
- Clippy strict mode enforced in CI (`-D warnings`).
- CI test execution stabilized via single-threaded test runs to avoid shared-state flakiness.

## 12) Current Runtime Mode

- Interactive shell/TUI runtime was removed.
- Shelldew currently starts as **MCP server runtime** by default.
