# Guest Piano Play Spec

## Status
Implemented in `src/piano.rs` and integrated in `src/main.rs`.

## Recent Updates
- **2026-03-26**: Updated to reflect piano moved from Square `(6,2)` to Farm `(4,2)`.

## Context
The guest girl can play the piano placed at Farm `(4,2)`. When the guest stands directly below the piano at `(4,3)`, keyboard keys activate piano notes with audible sound output. Only the guest has this ability — the player character cannot play the piano.

## Activation
- Guest must be enabled (`guest_enabled == true`).
- Guest must be in Farm region (`guest_location == Location::Farm`).
- Guest must be at tile `(4,3)` — directly south of the piano at `(4,2)`.
- When all conditions are met, the piano key bindings become active in the interactive input loop.
- Piano mode is passive — no explicit enter/exit action. Keys are simply available while standing at `(4,3)`.

## Key Mapping (Keyboard to Notes)
| Key | Note | Display | Key | Note | Display | Key | Note | Display |
|-----|------|---------|-----|------|---------|-----|------|---------|
| Z   | Do3  | C3      | A   | Do4  | C4      | Q   | Do5  | C5      |
| X   | Re3  | D3      | S   | Re4  | D4      | W   | Re5  | D5      |
| C   | Mi3  | E3      | D   | Mi4  | E4      | E   | Mi5  | E5      |
| V   | Fa3  | F#3     | F   | Fa4  | F#4     | R   | Fa5  | F#5     |
| B   | So3  | G3      | G   | So4  | G4      | T   | So5  | G5      |
| N   | La3  | A3      | H   | La4  | A4      | Y   | La5  | A5      |
| M   | Si3  | B3      | J   | Si4  | B4      | U   | Si5  | B5      |

- Keys are case-insensitive (`a`/`A` both trigger Do).
- Each key press plays the corresponding note sound and updates the bottom message.

## Message Display
- On key press, the bottom message line shows the note name:
  - Example: pressing `D` shows `🎵 Mi`
- The message updates on each key press, replacing the previous note display.
- If the guest moves away from `(4,3)`, the keys revert to their normal behavior (farm action rejection messages).

## Sound Playback
- See `piano-samples.spec.md` for complete sample file mapping and implementation details.

## Player Restriction
- When the player (non-guest) stands at `(4,3)`, the key bindings do **not** trigger piano notes.
- The piano interaction message from `north-square-piano.spec.md` still applies when walking into `(4,2)`.

## Implementation Notes
- **Input handling** (`src/main.rs`): Inside the guest-enabled branch of the interactive key loop, check if `guest_location == Farm && guest_x == 4 && guest_y == 3`. If true, intercept `KeyCode::Char('a'..'k', 'q'..'u', 'z'..'m')` for the 21 mapped keys before falling through to the default "Guest can only walk around." handler.
- **State** (`src/state.rs`): Add a method `guest_play_piano(&mut self, note: &str)` that sets `self.message` to the note display string (e.g., `"🎵 Mi"`). No persistent piano state is needed — each press is stateless.
- **Audio module**: `src/piano.rs` exposes `play_note(note: PianoNote)` that uses a dedicated audio thread pattern to avoid macOS Send/Sync issues. Audio initialization failures are silently ignored (sound is best-effort).
- **Cargo.toml**: `rodio` is added as a dependency with `rodio = { version = "0.21", optional = true }`. The `interactive` feature includes it.
- **No MCP impact**: Piano playback is interactive-only. The MCP command interface has no piano command and requires no changes.

## Related Specs
- `north-square-piano.spec.md` — Piano tile placement and walkability.
- `piano-samples.spec.md` — Sample file mapping and storage.
- `entities-and-movement.spec.md` — Guest movement and non-walkable tile rules.
- `farm.spec.md` — Farm map layout.
