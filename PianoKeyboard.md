# Piano Keyboard Mapping

Tinydew features two piano input systems that use the Salamander Grand Piano samples.

## 1. Block Key System (Full Keyboard)

The block key system uses keys Z through U to play a full range of notes (C3-B5).

### Full Key Mapping

| Key | Note | Sample File | Playback Speed | Key | Note | Sample File | Playback Speed |
|-----|------|-------------|----------------|-----|------|-------------|----------------|
| Z | C3 | C3v8.flac | 1.0 | Q | C5 | C5v8.flac | 1.0 |
| X | D3 | C3v8.flac | 1.1225 | W | D5 | C5v8.flac | 1.1225 |
| C | E3 | C3v8.flac | 1.2599 | E | E5 | C5v8.flac | 1.2599 |
| V | F3 | F3v8.flac | 0.9439 | R | F5 | F#5v8.flac | 0.9439 |
| B | G3 | F3v8.flac | 1.0595 | T | G5 | F#5v8.flac | 1.0595 |
| N | A3 | A3v8.flac | 1.0 | Y | A5 | A5v8.flac | 1.0 |
| M | B3 | A3v8.flac | 0.9439 | U | B5 | A5v8.flac | 0.9439 |
| A | C4 | C4v8.flac | 1.0 | | | | |
| S | D4 | C4v8.flac | 1.1225 | | | | |
| D | E4 | C4v8.flac | 1.2599 | | | | |
| F | F4 | F#4v8.flac | 0.9439 | | | | |
| G | G4 | F#4v8.flac | 1.0595 | | | | |
| H | A4 | A4v8.flac | 1.0 | | | | |
| J | B4 | A4v8.flac | 0.9439 | | | | |

### Usage

- Guest must be at position (6, 3) on the Square map (directly in front of the piano at 6, 2)
- Press any of the above keys to play the corresponding note
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
- C3v8.flac, C4v8.flac, C5v8.flac (root C notes, various speeds for pitch shifting)
- F3v8.flac, F#4v8.flac, F#5v8.flac (F/F# notes)
- A3v8.flac, A4v8.flac, A5v8.flac (A notes)

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
