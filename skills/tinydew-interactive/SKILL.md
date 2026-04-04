# tinydew — Interactive TUI Implementation Skill

## Context

tinydew is a cozy Rust farming game CLI. This skill covers implementing its interactive terminal UI mode where a guest girl (👧) walks around an emoji-based world using arrow keys.

## Project Structure

```
tinydew/
├── agents/                      # Spec documents (human-readable requirements)
│   ├── interactive-mode.spec.md # Main spec for TUI
│   ├── ui.spec.md               # UI rendering rules
│   ├── entities-and-movement.spec.md
│   ├── guest-piano-play.spec.md
│   ├── piano-samples.spec.md
│   └── ...
├── src/
│   ├── main.rs       # CLI entry, --interactive flag
│   ├── tui.rs        # Full-screen crossterm TUI loop
│   ├── piano.rs      # Piano audio via rodio
│   ├── block_key.rs  # Block key sounds
│   ├── map.rs        # Multi-region map definitions
│   ├── entity.rs     # Player entity, Direction enum
│   ├── state.rs      # GameState, save/load via JSON
│   ├── movement.rs   # Movement and region transitions
│   ├── ui.rs         # CLI status rendering
│   ├── cli.rs        # CLI action handlers (do/status)
│   └── ...           # economy, farming, weather, sleep, etc.
├── files/            # Piano sample .flac files (C3v8.flac..A5v8.flac)
├── Cargo.toml
└── tinydew_save.json # Persistent state
```

## Branch Workflow

- **`spec`** — Spec documents only (source of truth for implementation)
- **`engineering`** — Implementation branch, based on `spec`
- Read specs from `agents/`, implement on `engineering`, push when ready

## Interactive Mode

### Launch
```
cargo run --features interactive -- -i
```

### Controls
| Key | Action |
|-----|--------|
| `↑↓←→` | Move (arrow keys ONLY — no WASD/HJKL) |
| `Space` | Greet (region-specific) / Sleep (near house) |
| `Esc` / `Ctrl+C` | Save and exit |

### Piano Playing
- At Farm (4,3), letter keys Z/M, A/J, Q/U map to 21 notes (C3–B5)
- Arrow keys still move (walking away exits piano mode)
- Audio via `rodio 0.21` from `files/*.flac` samples

### TUI Rendering
- Full-screen crossterm: alternate screen, hidden cursor, raw mode
- Header: `tinydew day N <weather> HH:MM`
- Emoji map grid with 👧 player overlay
- Footer: message, inventory, money, region, controls
- Min terminal: 40×18, refuses to start otherwise

### Region Transitions
- Walking onto path tiles triggers seamless region switches
- Farm ↔ EastPath ↔ Square ↔ SouthRiver
- Each region has different map dimensions and tile layouts

### Build & Test
```bash
cargo build --features interactive  # must compile clean
cargo test                          # all 8 tests pass
```

## Common Gotchas

### rodio 0.21 API Changes
The API changed from older versions — use:
```rust
// OLD (broken):
let (_stream, handle) = OutputStream::try_default().unwrap();
let sink = Sink::try_new(&handle).unwrap();

// NEW (correct):
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
let handle = OutputStreamBuilder::open_default_stream().unwrap();
let mixer = handle.mixer();
let sink = Sink::connect_new(mixer);
```

Fix in: `src/piano.rs` and `src/block_key.rs`

### Weather emoji
Weather uses `Weather::icon(is_night)`, not `.emoji()`. Match directly or use the icon method.

### Inventory fields
Individual crop counts (carrots, strawberries, etc.), not a generic `produce`/`forage` field. Match the actual `Inventory` struct.

### Movement-only arrows
Do NOT use WASD or HJKL for movement — those letter keys are piano notes. Arrow keys only.

## Implementation Steps

1. **Read specs** — `agents/interactive-mode.spec.md` + related docs
2. **Fix rodio API** — Update `piano.rs` and `block_key.rs` for rodio 0.21
3. **Create `src/tui.rs`** — Full-screen crossterm TUI with arrow movement, greeting, piano mode
4. **Update `src/main.rs`** — Add `--interactive` / `-i` flag
5. **Build & test** — `cargo build --features interactive` + `cargo test`

## Git Conventions

- Commit messages end with `Co-authored-by: Claude Opus 4.6 <noreply@anthropic.com>`
- Push to `engineering` branch
- Branch based on `spec` for fresh starts
