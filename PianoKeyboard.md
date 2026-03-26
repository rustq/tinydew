# Piano Keyboard Mapping

Tinydew uses the PianoNote system to play notes across 3 octaves using the Salamander Grand Piano samples.

## Keyboard Mapping

| Key | Note | Key | Note | Key | Note |
|-----|------|-----|------|-----|------|
| Z | C3 | A | C4 | Q | C5 |
| X | D3 | S | D4 | W | D5 |
| C | E3 | D | E4 | E | E5 |
| V | F3 | F | F4 | R | F5 |
| B | G3 | G | G4 | T | G5 |
| N | A3 | H | A4 | Y | A5 |
| M | B3 | J | B4 | U | B5 |

### Octave Breakdown

**Lower Octave (Z-M):** C3 to B3
- Z, X, C → C3v8.flac
- V, B → F#3v8.flac
- N, M → A3v8.flac

**Middle Octave (A-J):** C4 to B4
- A, S, D → C4v8.flac
- F, G → F#4v8.flac
- H, J → A4v8.flac

**Upper Octave (Q-U):** C5 to B5
- Q, W, E → C5v8.flac
- R, T → F#5v8.flac
- Y, U → A5v8.flac

## Usage

- Guest must be at position (6, 3) on the Square map (directly in front of the piano at 6, 2)
- Press any of the above keys to play the corresponding note
- Notes are debounced - press and release to hear the note again
- Uses shared audio thread with max 4 concurrent notes

## Audio Implementation

- **Shared OutputStream**: A single background thread owns the audio stream
- **mpsc channel**: Key presses send `AudioCommand::Play` messages
- **Sink cleanup**: Automatic cleanup of finished notes, max 4 concurrent sinks
- **Sample format**: .flac files from Salamander Grand Piano project
- **Pitch shifting**: Uses playback speed ratios to transpose samples

## Sound Source

Uses the [Salamander Grand Piano](https://github.com/alexholm/salamander-grand-piano-in-rust) samples by Alexander Holm.

Available samples:
- C3v8.flac, C4v8.flac, C5v8.flac
- F#3v8.flac, F#4v8.flac, F#5v8.flac
- A3v8.flac, A4v8.flac, A5v8.flac

## Requirements

- Interactive mode must be enabled: `cargo build --features interactive`
- Audio system must be available (rodio uses system audio API via cpal)
- Sound samples must be present at: `../salamander-grand-piano-in-rust/Samples/`

## Building with Interactive Mode

```bash
cargo build --release --features interactive
```

Then run:

```bash
cargo run --features interactive
```

## Troubleshooting

If you see "Dropping OutputStream" warnings:
- Ensure only one audio stream is created
- The piano system uses a shared OutputStream for all notes
