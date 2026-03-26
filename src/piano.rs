use crossterm::event::KeyCode;

/// Represents a piano note that the guest can play at the North Square piano.
#[derive(Debug, Clone, Copy)]
pub enum PianoNote {
    Do,
    Re,
    Mi,
    Fa,
    So,
    La,
    Si,
    DoSharp,
}

impl PianoNote {
    /// Human-readable name displayed in the bottom message.
    pub fn display_name(self) -> &'static str {
        match self {
            PianoNote::Do => "Do",
            PianoNote::Re => "Re",
            PianoNote::Mi => "Mi",
            PianoNote::Fa => "Fa",
            PianoNote::So => "So",
            PianoNote::La => "La",
            PianoNote::Si => "Si",
            PianoNote::DoSharp => "DO#",
        }
    }

    /// File name for the Salamander Grand Piano sample used as the source.
    pub fn sample_file(self) -> &'static str {
        match self {
            PianoNote::Do | PianoNote::Re | PianoNote::Mi => "C4v8.flac",
            PianoNote::Fa | PianoNote::So => "F#4v8.flac",
            PianoNote::La => "A4v8.flac",
            PianoNote::Si | PianoNote::DoSharp => "C5v8.flac",
        }
    }

    /// Speed ratio applied to the source sample to reach the target pitch.
    pub fn playback_speed(self) -> f32 {
        match self {
            PianoNote::Do => 1.0,
            PianoNote::Re => 1.1225,
            PianoNote::Mi => 1.2599,
            PianoNote::Fa => 0.9439,
            PianoNote::So => 1.0595,
            PianoNote::La => 1.0,
            PianoNote::Si => 0.9439,
            PianoNote::DoSharp => 1.0,
        }
    }

    /// Converts an interactive key press into a `PianoNote`.
    pub fn from_key_code(key: KeyCode) -> Option<Self> {
        if let KeyCode::Char(ch) = key {
            match ch.to_ascii_lowercase() {
                'a' => Some(PianoNote::Do),
                's' => Some(PianoNote::Re),
                'd' => Some(PianoNote::Mi),
                'f' => Some(PianoNote::Fa),
                'g' => Some(PianoNote::So),
                'h' => Some(PianoNote::La),
                'j' => Some(PianoNote::Si),
                'k' => Some(PianoNote::DoSharp),
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
    use once_cell::sync::Lazy;
    use rodio::mixer::Mixer;
    use rodio::source::Source;
    use rodio::{Decoder, OutputStream, Sink};
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub fn play_note(note: PianoNote) {
        let manager = MANAGER.get_or_init(|| {
            Arc::new(Mutex::new(AudioManager::new().expect("Failed to initialize audio manager")))
        });
        manager.lock().unwrap().play(note);
    }

    struct AudioManager {
        _stream: OutputStream,
        mixer: Mutex<Mixer>,
        sinks: Mutex<VecDeque<Sink>>,
    }

    impl AudioManager {
        fn new() -> anyhow::Result<Self> {
            let stream = OutputStream::try_default()?;
            let mixer = Mutex::new(stream.mixer().clone());
            Ok(Self {
                _stream: stream,
                mixer,
                sinks: Mutex::new(VecDeque::new()),
            })
        }

        fn play(&self, note: PianoNote) {
            let path = Self::sample_path(note.sample_file());
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Failed to open piano sample {:?}: {}", path, err);
                    return;
                }
            };

            let decoder = match Decoder::new(BufReader::new(file)) {
                Ok(d) => d,
                Err(err) => {
                    eprintln!("Failed to decode piano sample {:?}: {}", path, err);
                    return;
                }
            };

            let speed = note.playback_speed();
            let source = decoder.speed(speed).fade_in(Duration::from_millis(8));

            let sink = Sink::connect_new(&self.mixer.lock().unwrap());

            sink.append(source);

            let mut sinks = self.sinks.lock().unwrap();
            sinks.retain(|s| !s.empty());
            if sinks.len() >= 4 {
                if let Some(old) = sinks.pop_front() {
                    old.stop();
                }
            }
            sinks.push_back(sink);
        }

        fn sample_path(file_name: &str) -> PathBuf {
            static SAMPLE_DIR: Lazy<PathBuf> = Lazy::new(|| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("salamander-grand-piano-in-rust")
                    .join("Samples")
            });
            SAMPLE_DIR.join(file_name)
        }
    }

    static MANAGER: Lazy<Arc<Mutex<AudioManager>>> = Lazy::new(|| {
        Arc::new(Mutex::new(AudioManager::new().expect("Failed to initialize audio manager")))
    });
}

#[cfg(feature = "interactive")]
pub use audio::play_note;

#[cfg(not(feature = "interactive"))]
pub fn play_note(_: PianoNote) {}
