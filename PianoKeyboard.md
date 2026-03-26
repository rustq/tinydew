# Piano Keyboard Mapping

Tinydew features two piano input systems:

## 1. Block Key System (QWERTYUIOP)

The default piano system uses keyboard keys Q, W, E, R, T, Y, U, I, O, P to play notes.

| Key | Note | Frequency | Sample File | Playback Speed |
|-----|------|-----------|-------------|----------------|
| Q | C4 | 261.63 Hz | C4v8.flac | 1.0 |
| W | D4 | 293.66 Hz | C4v8.flac | 1.1225 |
| E | E4 | 329.63 Hz | C4v8.flac | 1.2599 |
| R | F4 | 349.23 Hz | C4v8.flac | 0.9439 |
| T | G4 | 392.00 Hz | C4v8.flac | 1.0595 |
| Y | A4 | 440.00 Hz | A4v8.flac | 1.0 |
| U | B4 | 493.88 Hz | A4v8.flac | 0.9439 |
| I | C5 | 523.25 Hz | C5v8.flac | 1.0 |
| O | D5 | 587.33 Hz | C5v8.flac | 1.1225 |
| P | E5 | 659.25 Hz | C5v8.flac | 1.2599 |

### Usage

- Guest must be at position (6, 3) on the Square map (directly in front of the piano at 6, 2)
- Press any of the above keys to play the corresponding note
- Keys are debounced - press and release to hear the note again

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

## Sound Source

Both systems use the [Salamander Grand Piano](https://github.com/alexholm/salamander-grand-piano-in-rust) samples by Alexander Holm.

## Requirements

- Interactive mode must be enabled: `cargo build --features interactive`
- Audio system must be available (rodio uses system audio API)
- Sound samples must be present at: `../salamander-grand-piano-in-rust/Samples/`

---

## Building with Interactive Mode

```bash
cargo build --release --features interactive
```

Then run:

```bash
cargo run --features interactive
```
