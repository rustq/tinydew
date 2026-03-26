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
