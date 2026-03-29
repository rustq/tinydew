# Piano Sample Files Spec

## Status
Documented.

## Sample Source
Piano samples come from the **Salamander Grand Piano v3** by Alexander Holm (CC-BY license).
The sample files are stored in `./files` directory at the project root.

### Sample Files Required
Only **9 source sample files** are needed. Each file contains a single note recorded at medium velocity (v8). Additional notes are derived via pitch-shifting:

| Sample File | Base Note | Used For (Octave 3) | Used For (Octave 4) | Used For (Octave 5) |
|-------------|-----------|---------------------|---------------------|---------------------|
| `C3v8.flac` | C3 | Do3, Re3, Mi3 | Do4, Re4, Mi4 | Do5, Re5, Mi5 |
| `F#3v8.flac` | F#3 | Fa3, So3 | - | - |
| `A3v8.flac` | A3 | La3, Si3 | - | - |
| `C4v8.flac` | C4 | - | Do4, Re4, Mi4 | - |
| `F#4v8.flac` | F#4 | - | Fa4, So4 | - |
| `A4v8.flac` | A4 | - | La4, Si4 | - |
| `C5v8.flac` | C5 | - | - | Do5, Re5, Mi5 |
| `F#5v8.flac` | F#5 | - | - | Fa5, So5 |
| `A5v8.flac` | A5 | - | - | La5, Si5 |

### Full Note-to-Sample Mapping

#### Lower Octave (C3-B3)
| Note | Display | Sample File | Semitone Offset | Speed Ratio |
|------|---------|-------------|-----------------|-------------|
| Do3 (C3) | C3 | C3v8.flac | 0 | 1.0 |
| Re3 (D3) | D3 | C3v8.flac | +2 | 1.1225 |
| Mi3 (E3) | E3 | C3v8.flac | +4 | 1.2599 |
| Fa3 (F#3) | F#3 | F#3v8.flac | 0 | 1.0 |
| So3 (G3) | G3 | F#3v8.flac | +1 | 1.0595 |
| La3 (A3) | A3 | A3v8.flac | 0 | 1.0 |
| Si3 (B3) | B3 | A3v8.flac | +1 | 1.0595 |

#### Middle Octave (C4-B4)
| Note | Display | Sample File | Semitone Offset | Speed Ratio |
|------|---------|-------------|-----------------|-------------|
| Do4 (C4) | C4 | C4v8.flac | 0 | 1.0 |
| Re4 (D4) | D4 | C4v8.flac | +2 | 1.1225 |
| Mi4 (E4) | E4 | C4v8.flac | +4 | 1.2599 |
| Fa4 (F#4) | F#4 | F#4v8.flac | 0 | 1.0 |
| So4 (G4) | G4 | F#4v8.flac | +1 | 1.0595 |
| La4 (A4) | A4 | A4v8.flac | 0 | 1.0 |
| Si4 (B4) | B4 | A4v8.flac | +1 | 1.0595 |

#### Upper Octave (C5-B5)
| Note | Display | Sample File | Semitone Offset | Speed Ratio |
|------|---------|-------------|-----------------|-------------|
| Do5 (C5) | C5 | C5v8.flac | 0 | 1.0 |
| Re5 (D5) | D5 | C5v8.flac | +2 | 1.1225 |
| Mi5 (E5) | E5 | C5v8.flac | +4 | 1.2599 |
| Fa5 (F#5) | F#5 | F#5v8.flac | 0 | 1.0 |
| So5 (G5) | G5 | F#5v8.flac | +1 | 1.0595 |
| La5 (A5) | A5 | A5v8.flac | 0 | 1.0 |
| Si5 (B5) | B5 | A5v8.flac | +1 | 1.0595 |

### Key Mapping (Keyboard)
| Key | Note | Display | Key | Note | Display |
|-----|------|---------|-----|------|---------|
| Z | Do3 | C3 | A | Do4 | C4 | Q | Do5 | C5 |
| X | Re3 | D3 | S | Re4 | D4 | W | Re5 | D5 |
| C | Mi3 | E3 | D | Mi4 | E3 | E | Mi5 | E5 |
| V | Fa3 | F#3 | F | Fa4 | F#4 | R | Fa5 | F#5 |
| B | So3 | G3 | G | So4 | G4 | T | So5 | G5 |
| N | La3 | A3 | H | La4 | A4 | Y | La5 | A5 |
| M | Si3 | B3 | J | Si4 | B4 | U | Si5 | B5 |

### Sample Storage
Sample files should be stored in `./files` directory at the project root.

Required files (9 total):
```
./files/C3v8.flac
./files/F#3v8.flac
./files/A3v8.flac
./files/C4v8.flac
./files/F#4v8.flac
./files/A4v8.flac
./files/C5v8.flac
./files/F#5v8.flac
./files/A5v8.flac
```

### Playback Engine
- Uses `rodio` crate for audio decoding and playback (cross-platform: ALSA/PulseAudio on Linux, CoreAudio on macOS, WASAPI on Windows).
- **Audio thread pattern**: A dedicated thread owns the `OutputStream` to avoid macOS `Send` trait issues.
- **Polyphony**: Up to 4 concurrent voice sinks; 5th note stops the oldest.
- **Fade-in**: 8ms fade-in applied per note to prevent click artifacts.

### License
Salamander Grand Piano v3 by Alexander Holm - CC-BY license.
