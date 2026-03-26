use crossterm::event::KeyCode;

/// Represents a piano note across 3 octaves (C3-B5).
#[derive(Debug, Clone, Copy)]
pub enum PianoNote {
    // Lower Octave (C3-B3)
    Do3,
    Re3,
    Mi3,
    Fa3,
    So3,
    La3,
    Si3,
    // Middle Octave (C4-B4)
    Do4,
    Re4,
    Mi4,
    Fa4,
    So4,
    La4,
    Si4,
    // Upper Octave (C5-B5)
    Do5,
    Re5,
    Mi5,
    Fa5,
    So5,
    La5,
    Si5,
}

impl PianoNote {
    /// Human-readable name displayed in the bottom message.
    pub fn display_name(self) -> &'static str {
        match self {
            PianoNote::Do3 => "C3",
            PianoNote::Re3 => "D3",
            PianoNote::Mi3 => "E3",
            PianoNote::Fa3 => "F3",
            PianoNote::So3 => "G3",
            PianoNote::La3 => "A3",
            PianoNote::Si3 => "B3",
            PianoNote::Do4 => "C4",
            PianoNote::Re4 => "D4",
            PianoNote::Mi4 => "E4",
            PianoNote::Fa4 => "F4",
            PianoNote::So4 => "G4",
            PianoNote::La4 => "A4",
            PianoNote::Si4 => "B4",
            PianoNote::Do5 => "C5",
            PianoNote::Re5 => "D5",
            PianoNote::Mi5 => "E5",
            PianoNote::Fa5 => "F5",
            PianoNote::So5 => "G5",
            PianoNote::La5 => "A5",
            PianoNote::Si5 => "B5",
        }
    }

    /// File name for the Salamander Grand Piano sample used as the source.
    pub fn sample_file(self) -> &'static str {
        match self {
            PianoNote::Do3 | PianoNote::Re3 | PianoNote::Mi3 => "C3v8.flac",
            PianoNote::Fa3 | PianoNote::So3 => "F#3v8.flac",
            PianoNote::La3 | PianoNote::Si3 => "A3v8.flac",
            PianoNote::Do4 | PianoNote::Re4 | PianoNote::Mi4 => "C4v8.flac",
            PianoNote::Fa4 | PianoNote::So4 => "F#4v8.flac",
            PianoNote::La4 | PianoNote::Si4 => "A4v8.flac",
            PianoNote::Do5 | PianoNote::Re5 | PianoNote::Mi5 => "C5v8.flac",
            PianoNote::Fa5 | PianoNote::So5 => "F#5v8.flac",
            PianoNote::La5 | PianoNote::Si5 => "A5v8.flac",
        }
    }

    /// Speed ratio applied to the source sample to reach the target pitch.
    pub fn playback_speed(self) -> f32 {
        match self {
            PianoNote::Do3 => 1.0,
            PianoNote::Re3 => 1.1225,
            PianoNote::Mi3 => 1.2599,
            PianoNote::Fa3 => 0.9439,
            PianoNote::So3 => 1.0595,
            PianoNote::La3 => 1.0,
            PianoNote::Si3 => 1.1225,
            PianoNote::Do4 => 1.0,
            PianoNote::Re4 => 1.1225,
            PianoNote::Mi4 => 1.2599,
            PianoNote::Fa4 => 0.9439,
            PianoNote::So4 => 1.0595,
            PianoNote::La4 => 1.0,
            PianoNote::Si4 => 1.1225,
            PianoNote::Do5 => 1.0,
            PianoNote::Re5 => 1.1225,
            PianoNote::Mi5 => 1.2599,
            PianoNote::Fa5 => 0.9439,
            PianoNote::So5 => 1.0595,
            PianoNote::La5 => 1.0,
            PianoNote::Si5 => 1.1225,
        }
    }

    /// Converts an interactive key press into a `PianoNote`.
    pub fn from_key_code(key: KeyCode) -> Option<Self> {
        if let KeyCode::Char(ch) = key {
            match ch.to_ascii_lowercase() {
                // Lower Octave (Z-M)
                'z' => Some(PianoNote::Do3),
                'x' => Some(PianoNote::Re3),
                'c' => Some(PianoNote::Mi3),
                'v' => Some(PianoNote::Fa3),
                'b' => Some(PianoNote::So3),
                'n' => Some(PianoNote::La3),
                'm' => Some(PianoNote::Si3),
                // Middle Octave (A-J)
                'a' => Some(PianoNote::Do4),
                's' => Some(PianoNote::Re4),
                'd' => Some(PianoNote::Mi4),
                'f' => Some(PianoNote::Fa4),
                'g' => Some(PianoNote::So4),
                'h' => Some(PianoNote::La4),
                'j' => Some(PianoNote::Si4),
                // Upper Octave (Q-U)
                'q' => Some(PianoNote::Do5),
                'w' => Some(PianoNote::Re5),
                'e' => Some(PianoNote::Mi5),
                'r' => Some(PianoNote::Fa5),
                't' => Some(PianoNote::So5),
                'y' => Some(PianoNote::La5),
                'u' => Some(PianoNote::Si5),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "interactive")]
mod audio {
    use super::PianoNote;
    use rodio::source::Source;
    use rodio::{Decoder, OutputStreamBuilder, Sink};
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::sync::mpsc;
    use std::thread;

    pub fn play_note(note: PianoNote) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("salamander-grand-piano-in-rust")
            .join("Samples")
            .join(note.sample_file());
        let speed = note.playback_speed();
        let source_data = std::fs::read(&path).ok();

        if let Some(source_data) = source_data {
            // Send the note info to the audio thread via channel
            let _ = AUDIO_SENDER.send(AudioCommand::Play {
                source_data,
                speed,
            });
        }
    }

    #[derive(Debug)]
    enum AudioCommand {
        Play {
            source_data: Vec<u8>,
            speed: f32,
        },
        Stop,
    }

    // Channel sender for audio commands
    static AUDIO_SENDER: once_cell::sync::Lazy<mpsc::Sender<AudioCommand>> = once_cell::sync::Lazy::new(|| {
        let (tx, rx) = mpsc::channel::<AudioCommand>();

        // Spawn audio thread that owns the OutputStream
        thread::spawn(move || {
            let Ok(stream) = OutputStreamBuilder::open_default_stream() else {
                eprintln!("Failed to open audio stream");
                return;
            };
            let mixer = stream.mixer().clone();

            let mut sinks: VecDeque<Sink> = VecDeque::new();

            for cmd in rx {
                match cmd {
                    AudioCommand::Play { source_data, speed } => {
                        let cursor = std::io::Cursor::new(source_data);
                        if let Ok(source) = Decoder::new(cursor) {
                            let source = source
                                .speed(speed)
                                .fade_in(std::time::Duration::from_millis(8));

                            let sink = Sink::connect_new(&mixer);
                            sink.append(source);
                            sinks.push_back(sink);
                        }
                    }
                    AudioCommand::Stop => {
                        for sink in sinks.drain(..) {
                            sink.stop();
                        }
                    }
                }

                // Clean up empty sinks
                sinks.retain(|s| !s.empty());

                // Limit concurrent sinks
                if sinks.len() > 4 {
                    if let Some(old) = sinks.pop_front() {
                        old.stop();
                    }
                }
            }
        });

        tx
    });
}

#[cfg(feature = "interactive")]
pub use audio::play_note;

#[cfg(not(feature = "interactive"))]
pub fn play_note(_: PianoNote) {}
