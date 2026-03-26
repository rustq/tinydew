# Block Key Sound System — Spec

## Status
Implemented in `src/block_key.rs`.

## Overview

A keyboard-driven sound instrument where each of the 10 keys `Q W E R T Y U I O P` triggers a distinct musical note sound on keydown. This is a one-row piano/sampler for the Rust CLI app.

## Key-to-Sound Mapping (Option A - C Major Scale)

| Key | Note | Frequency (Hz) |
|-----|------|----------------|
| Q   | C4   | 261.63         |
| W   | D4   | 293.66         |
| E   | E4   | 329.63         |
| R   | F4   | 349.23         |
| T   | G4   | 392.00         |
| Y   | A4   | 440.00         |
| U   | B4   | 493.88         |
| I   | C5   | 523.25         |
| O   | D5   | 587.33         |
| P   | E5   | 659.25         |

## Audio Playback Mechanism

**Rust rodio crate** — cross-platform audio playback.

- On keydown → generate a sine wave at the mapped frequency
- Create a 500ms duration tone with simple envelope
- Apply gain (0.3 volume) to prevent clipping
- Each key press is fire-and-forget (no keyup handling needed)
- Audio initialization failures are silently ignored

### Event Flow

```
key_code = KeyCode::Char('q')
  → BlockKeyNote::Q(C4) generated from key code
  → play_note(note) spawns new audio thread
  → Generate 500ms sine wave at 261.63Hz
  → Play via rodio::Sink
```

## State Management

```rust
// No persistent state needed - each press is stateless.
// Active notes are managed internally by rodio's Sink.

// Key tracking is done via Set in the input handler
// to prevent re-triggering on key repeat.
```

### Key Concerns

1. **Debounce key repeat** — track pressed keys in a `Set` to ignore key held events
2. **Audio initialization** — silently fail if no audio device available
3. **Polyphony** — multiple keys create separate sinks (handled by rodio)
4. **No cleanup** — rodio handles sink cleanup automatically
5. **Case insensitivity** — normalize `key.to_ascii_lowercase()` to handle Caps Lock

## Implementation Notes

- **Input handling** (`src/main.rs`): In the interactive key loop, intercept `KeyCode::Char('q'..='p')` for the 10 mapped keys
- **Note mapping** (`src/block_key.rs`): `BlockKeyNote::from_key_code(key)` converts `KeyCode` to `BlockKeyNote`
- **Audio module**: `src/block_key.rs` exposes `play_note(note)` which generates and plays a sine wave via rodio
- **Cargo.toml**: `rodio` is already added as a dependency (from piano.rs)

## Related Specs

- `guest-piano-play.spec.md` — Existing piano implementation for reference
- `north-square-piano.spec.md` — Piano tile placement and walkability
