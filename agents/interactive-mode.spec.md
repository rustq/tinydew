# Interactive Mode Spec

## Status

Planned — extends the existing interactive TUI path. Currently no `--interactive` flag exists; this spec defines it end-to-end.

## Overview

A real-time, keyboard-driven interactive mode where the player controls a **guest girl** character walking around tinydew worlds using arrow keys. Press `Esc` to exit.

## How to Launch

```
tinydew --interactive
```

Cargo feature required:
```toml
[features]
interactive = ["crossterm", "rodio"]
```

This enables two optional dependencies:
- **crossterm** — raw terminal + key event reading + cursor control
- **rodio** — audio playback for piano notes and sound effects

## Game Mode: Guest Girl

In interactive mode the player is always the **guest girl** (👧). This is a deliberate design choice — the player explores the world as a visitor, not the farm owner.

### Guest Entity Properties

- Emoji: `👧`
- Starts at Farm `(3,3)` — the default wake position
- Independent `x` / `y` / `location` / `direction` fields
- Follows the same walkability and transition rules as any entity

## Controls

### Global

| Key          | Action                       |
| ------------ | ---------------------------- |
| `↑`          | Move up                      |
| `↓`          | Move down                    |
| `←`          | Move left                    |
| `→`          | Move right                   |
| `Space`      | Greet (region-specific text) |
| `Esc`        | Exit interactive mode        |

**Arrow keys only for movement.** No WASD / HJKL — all letter keys are reserved for piano and future interactions.

Piano mode activates when standing directly below the piano — see [Piano Playing](#piano-playing).

### Walkability Rules

Follows existing `TileType::is_walkable()` logic — guest cannot walk onto:

- Boundary (🌳), House (🏠), Piano (🎹), Fountain (⛲), Slide (🛝), River (🌊/🫧), Mushroom (🍄), Wonder (🦋)
- Mature crop tiles (🥕/🍓/🥦/🌺 — fully grown)

Blocked movement shows a brief message at the bottom and does **not** advance time.

Successful moves advance time by 5 minutes (same as CLI `do move`).

### Region Transitions

Walking onto path tiles triggers seamless region switches with spawn-point mapping:


## TUI Rendering

The screen is a full-screen crossterm application that redraws on every input.

### Layout

```
┌──────────────────────────────────┐
│ tinydew day 1 ☀️  06:20          │  ← Header (weather + time)
├──────────────────────────────────┤
│                                  │
│  🌳🌳🌳🌳🌳🌳🌳🌳🌳           │
│  🌳🌿🌿🌿🌿🌿🌿🌳🌳           │  ← Map emoji grid
│  🌳🌿🏠🌿👧🌿🌿🌳🌳           │     (guest at 4,2)
│  🌳🌿🌿🌿🌿🌿🌿🌳🌳           │
│  🌳🌳🌳🌳🌳🌳🌳🌳🌳           │
│                                  │
├──────────────────────────────────┤
│ > Hello!                         │  ← Message line (> prefix)
│ 🫙 x5     💰 $100               │  ← Inventory + money
│ Farm                             │  ← Current region name
│ ↑↓←→ move  Space greet  Esc quit│  ← Controls hint
└──────────────────────────────────┘
```

### Rendering Layers

1. **Title bar** — `tinydew day <N> <weather_icon>  HH:MM`
2. **Map grid** — emoji-per-tile, guest overlay replaces tile emoji
3. **Message line** — `> <message>` (last action feedback or contextual)
4. **Footer strip** — inventory summary, region name, control hints

### Redraw Strategy

- Full screen redraw on every key press
- `crossterm::execute!(EnterAlternateScreen, EnableMouseCapture)` on startup
- Hide cursor (`crossterm::execute!(Hide)`) — player is rendered via emoji grid

### Terminal Dimensions

- Minimum width: 40 columns (to fit the widest map: SouthRiver at 13 cols padding)
- Minimum height: 18 rows (header + 13-row map + footer)
- If terminal too small, show `"Terminal too small (min 40x18)"` and refuse to render.

## Piano Playing

### Activation

When guest is at **Farm `(4,3)`** — directly south of the piano tile at `(4,2)` — the piano keyboard activates.

The controls hint line swaps to show piano keys:

```
Z X C V B N M | A S D F G H J | Q W E R T Y U → play piano   Esc quit
```

### Piano Key Mapping

| Row Key | Octave 3 | Octave 4 | Octave 5 |
| -------: | :-------- | :-------- | :-------- |
| 1st col  | Z → C3    | A → C4    | Q → C5    |
| 2nd col  | X → D3    | S → D4    | W → D5    |
| 3rd col  | C → E3    | D → E4    | E → E5    |
| 4th col  | V → F#3   | F → F#4   | R → F#5   |
| 5th col  | B → G3    | G → G4    | T → G5    |
| 6th col  | N → A3    | H → A4    | Y → A5    |
| 7th col  | M → B3    | J → B4    | U → B5    |

### Piano Mode Behaviors

In piano mode (guest at Farm `(4,3)`):

- **Arrow keys still work** — moving away from `(4,3)` immediately exits piano mode
- **Space still works** — greeting still available
- **Piano note keys** play audio via `rodio` and show the note emoji in the message line
- Keys are case-insensitive
- Each note plays instantly with no debouncing (rapid play supported)
- Audio files loaded from `files/` directory (see `piano-samples.spec.md`)

### Audio

- Uses `rodio 0.21` for sound playback
- 21 sample files: `C3v8.flac` through `A5v8.flac`
- Each note triggers a separate audio thread (fire-and-forget) — overlapping notes play simultaneously
- Audio init failures are silently swallowed (piano is visual-only if no audio device)

### Message in Piano Mode

Pressing a piano key replaces the message line with the note name:
```
> 🎵 Mi (E4)
```
The previous message is restored after 2 seconds or the next non-piano action.

## Greeting

Pressing `Space` shows a region-specific greeting:

| Region     | Guest Greeting                       |
| ---------- | ------------------------------------ |
| Farm       | `Hello!`                             |
| EastPath   | `This path leads somewhere...`       |
| Square     | `✨ The fountain sparkles!`          |
| SouthRiver | `The river flows quietly today.`     |

Greeting text replaces the message line for 3 seconds or until the next non-greet action.

## Time & Day

- Time advances 5 minutes per successful movement
- Sleeping is done by returning to the House tile and pressing `Space` near it
- After sleeping: day increments, time resets to 06:00, weather re-rolls
- Weather: ☀️ sunny (default), 🌧️ rainy (random)
- Day 28 (Spring) = Butterfly Festival — special message overrides

## Input Loop

```
initialize GameState (load or fresh)
set up crossterm (alternate screen, hidden cursor, raw input)
loop:
  render TUI
  wait for key event with 100ms timeout (poll)
  match key:
    Arrow keys → movement
    Space → greeting
    Piano keys (when in piano mode) → play note
    Esc → break loop (save and exit)
  update time if movement was successful
  persist state to savegame.json
restore terminal (leave alternate screen, show cursor)
```

## Terminal Cleanup

On exit (Esc or Ctrl+C):

1. Save current state to `savegame.json`
2. Restore terminal: leave alternate screen, show cursor, disable raw mode
3. Print final status line to stdout

## Error Handling

| Scenario                         | Behavior                          |
| -------------------------------- | --------------------------------- |
| Terminal too small               | Print min size message, exit 0    |
| Audio device unavailable         | Silent fail, piano still visual   |
| Save file corrupted/missing      | Start fresh (Day 1, Farm, $100)   |
| Unmapped key pressed             | No action (ignored silently)      |
| Ctrl+C / SIGINT                  | Save and clean exit (same as Esc) |

## Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rand = "0.8"
crossterm = { version = "0.28", optional = true }
rodio = { version = "0.21", optional = true }
```

## Feature Flag

The entire interactive mode is gated behind the `interactive` cargo feature:

```toml
[features]
default = []
interactive = ["crossterm", "rodio"]
```

Without `--features interactive`, the binary ships as CLI-only (`tinydew status` / `tinydew do ...` only).

## Related Specs

- `entities-and-movement.spec.md` — Guest movement and walkability rules
- `farm-piano.spec.md` — Piano tile placement and interaction
- `guest-piano-play.spec.md` — Piano key mapping and note playback
- `piano-samples.spec.md` — Audio sample files
- `ui.spec.md` — TUI rendering format
- `economy-and-items.spec.md` — Inventory and money
- `cli.spec.md` — CLI interface (non-interactive)