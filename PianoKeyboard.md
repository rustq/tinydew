# Piano Keyboard Mapping

Tinydew features two piano input systems that use the Salamander Grand Piano samples.

## 1. Block Key System (QWERTYUIOP) - Default

The default piano system uses keyboard keys Q, W, E, R, T, Y, U, I, O, P to play notes.

| Key | Note | Sample File | Playback Speed | Description |
|-----|------|-------------|----------------|-------------|
| Q | C# | C4v8.flac | 1.0595 | Black key |
| W | D# | C4v8.flac | 1.1892 | Black key |
| E | F# | F#4v8.flac | 1.0 | Black key |
| R | G# | F#4v8.flac | 1.0595 | Black key |
| T | A# | A4v8.flac | 1.1892 | Black key |
| Y | No sound | - | - | Empty |
| U | No sound | - | - | Empty |
| I | No sound | - | - | Empty |
| O | No sound | - | - | Empty |
| P | No sound | - | - | Empty |

### Usage

- Guest must be at position (6, 3) on the Square map (directly in front of the piano at 6, 2)
- Press Q, W, E, R, T to play black keys (C#, D#, F#, G#, A#)
- Y, U, I, O, P produce no sound
- Keys are debounced - press and release to hear the note again
- Uses shared audio thread to prevent "Dropping OutputStream" warnings

## 2. Traditional Piano System (ASDFGHJK)

A secondary piano system uses keys A, S, D, F, G, H, J, K.

| Key | Note | Sample File | Playback Speed |
|-----|------|-------------|----------------|
| A | Do (C4) | C4v8.flac | 1.0 |
| S | Re (D4) | C4v8.flac | 1.1225 |
| D | Mi (E4) | C4v8.flac | 1.2599 |
| F | Fa (F4) | F#4v8.flac | 0.9439 |
| G | So (G4) | F#4v8.flac | 1.0595 |
| H | La (A4) | A4v8.flac | 1.0 |
| J | Si (B4) | A4v8.flac | 0.9439 |
| K | Do# (C5) | C5v8.flac | 1.0 |

### Usage

- Same position requirement: guest at (6, 3) on Square map
- Uses the same `PianoNote` type as the traditional keyboard layout

## Audio Implementation

Both systems use identical audio logic:

- **Shared OutputStream**: A single background thread owns the audio stream
- **mpsc channel**: Key presses send `AudioCommand::Play` messages
- **Sink cleanup**: Automatic cleanup of finished notes, max 4 concurrent sinks
- **Sample format**: .flac files from Salamander Grand Piano project

## Sound Source

Both systems use the [Salamander Grand Piano](https://github.com/alexholm/salamander-grand-piano-in-rust) samples by Alexander Holm.

Available samples:
- C4v8.flac (root C4, various speeds for pitch shifting)
- F#4v8.flac (F#4 for Fa and So notes)
- A4v8.flac (A4 for La note)
- C5v8.flac (C5 for higher notes)

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
- Ensure only one audio stream is created (fixed in recent versions)
- The block key system now uses a shared OutputStream for all notes
