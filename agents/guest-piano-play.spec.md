# Guest Piano Play Spec

## Status
Implemented in `src/piano.rs` and integrated in `src/main.rs`.

## Recent Updates
- **2026-03-26**: Updated to reflect macOS compatibility fix. Audio thread owns `OutputStream` to avoid `Send` trait issues with CoreAudio's non-Send `PropertyListenerCallbackWrapper`. Uses `mpsc` channel for communication between main thread and audio thread.

## Context
The guest girl can play the piano placed at Farm `(4,2)`. When the guest stands directly below the piano at `(4,3)`, keyboard keys activate piano notes with audible sound output. Only the guest has this ability — the player character cannot play the piano.

## Activation
- Guest must be enabled (`guest_enabled == true`).
- Guest must be in Farm region (`guest_location == Location::Farm`).
- Guest must be at tile `(4,3)` — directly south of the piano at `(4,2)`.
- When all conditions are met, the piano key bindings become active in the interactive input loop.
- Piano mode is passive — no explicit enter/exit action. Keys are simply available while standing at `(6,3)`.

## Key Mapping
| Key | Note Name | Display |
|-----|-----------|---------|
| A   | Do        | Do      |
| S   | Re        | Re      |
| D   | Mi        | Mi      |
| F   | Fa        | Fa      |
| G   | So        | So      |
| H   | La        | La      |
| J   | Si        | Si      |
| K   | DO#       | DO#     |

- Keys are case-insensitive (`a`/`A` both trigger Do).
- Each key press plays the corresponding note sound and updates the bottom message.

## Message Display
- On key press, the bottom message line shows the note name:
  - Example: pressing `D` shows `🎵 Mi`
- The message updates on each key press, replacing the previous note display.
- If the guest moves away from `(6,3)`, the keys revert to their normal behavior (farm action rejection messages).

## Sound Playback

### Sample Source
- Piano samples come from the **Salamander Grand Piano v3** by Alexander Holm (CC-BY license).
- Sample format: FLAC files, naming convention `[Note][Octave]v[Velocity].flac` (e.g., `C4v8.flac`, `A4v8.flac`).
- Only 5 source samples are needed (the direct-hit notes); the remaining notes are derived via pitch-shifting:
  - **C4v8.flac** — used for Do (C4), Re (D4, +2 semitones), Mi (E4, +4 semitones)
  - **F#4v8.flac** — used for Fa (F4, −1 semitone), So (G4, +1 semitone)
  - **A4v8.flac** — used for La (A4, direct)
  - **C5v8.flac** — used for Si (B4, −1 semitone), DO# (C5, direct)
- Velocity layer v8 (medium) is used for all notes — a single dynamic level is sufficient.
- Samples are stored in a `Samples/` directory at the project root.

### Note-to-Pitch Mapping
| Note | Display | Source Sample | Semitone Offset | Speed Ratio (`2^(offset/12)`) |
|------|---------|---------------|-----------------|-------------------------------|
| Do   | Do      | C4v8.flac     | 0               | 1.0                           |
| Re   | Re      | C4v8.flac     | +2              | 1.1225                        |
| Mi   | Mi      | C4v8.flac     | +4              | 1.2599                        |
| Fa   | Fa      | F#4v8.flac    | −1              | 0.9439                        |
| So   | So      | F#4v8.flac    | +1              | 1.0595                        |
| La   | La      | A4v8.flac     | 0               | 1.0                           |
| Si   | Si      | C5v8.flac     | −1              | 0.9439                        |
| DO#  | DO#     | C5v8.flac     | 0               | 1.0                           |

### Playback Engine
- Uses `rodio` crate for audio decoding and playback (cross-platform: ALSA/PulseAudio on Linux, CoreAudio on macOS, WASAPI on Windows).
- **Audio thread pattern**: A dedicated thread owns the `OutputStream` to avoid macOS `Send` trait issues (CoreAudio's `PropertyListenerCallbackWrapper` is not `Send`). The main thread communicates via `mpsc` channel.
- Playback pipeline per note:
  1. Main thread reads sample bytes from disk and sends via channel.
  2. Audio thread receives, wraps in `std::io::Cursor`, decodes with `rodio::Decoder::new()`.
  3. Applies pitch shift via `.speed(speed_ratio)` using the semitone formula `2.0_f32.powf(offset / 12.0)`.
  4. Applies a short fade-in (`.fade_in(Duration::from_millis(8))`) to prevent click artifacts.
  5. Appends to a `rodio::Sink` for non-blocking playback.
- **Polyphony**: Maintain up to 4 concurrent voice sinks in a `VecDeque<Sink>`. When a 5th note triggers, stop the oldest sink. Prune empty sinks on each key press.
- Each key press triggers a single note playback (non-blocking, fire-and-forget).
- If audio initialization fails (e.g., no audio device), the game continues silently — sound is best-effort, never a hard failure.

## Player Restriction
- When the player (non-guest) stands at `(4,3)`, the Z/U keys do **not** trigger piano notes.
- The piano interaction message from `farm-piano.spec.md` still applies when walking into `(4,2)`.

## Implementation Notes
- **Input handling** (`src/main.rs`): Inside the guest-enabled branch of the interactive key loop, check if `guest_location == Farm && guest_x == 4 && guest_y == 3`. If true, intercept `KeyCode::Char('a'..'u')` for all mapped keys before falling through to the default "Guest can only walk around." handler.
- **State** (`src/state.rs`): Add a method `guest_play_piano(&mut self, note: &str)` that sets `self.message` to the note display string (e.g., `"🎵 Do"`). No persistent piano state is needed — each press is stateless.
- **Audio module**: `src/piano.rs` exposes `play_note(note: PianoNote)` that uses a dedicated audio thread pattern to avoid macOS Send/Sync issues. Audio initialization failures are silently ignored (sound is best-effort).
- **Cargo.toml**: `rodio` is added as a dependency with `rodio = { version = "0.21", optional = true }`. The `interactive` feature includes it.
- **No MCP impact**: Piano playback is interactive-only. The MCP command interface has no piano command and requires no changes.

## Related Specs
- `farm-piano.spec.md` — Piano tile placement and walkability.
- `entities-and-movement.spec.md` — Guest movement and non-walkable tile rules.
- `farm-region.spec.md` — Farm map layout.
