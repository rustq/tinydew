use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crossterm::event::KeyCode;
use once_cell::sync::Lazy as OnceCellLazy;
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::io::Cursor;

/// Musical notes mapped to keyboard keys (C Major scale, C3-B5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKeyNote {
    C3,
    D3,
    E3,
    F3,
    G3,
    A3,
    B3,
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    C5,
    D5,
    E5,
    F5,
    G5,
    A5,
    B5,
    None, // No sound
}

impl BlockKeyNote {
    /// Human-readable name displayed in the bottom message.
    pub fn display_name(self) -> &'static str {
        match self {
            BlockKeyNote::C3 => "C3",
            BlockKeyNote::D3 => "D3",
            BlockKeyNote::E3 => "E3",
            BlockKeyNote::F3 => "F3",
            BlockKeyNote::G3 => "G3",
            BlockKeyNote::A3 => "A3",
            BlockKeyNote::B3 => "B3",
            BlockKeyNote::C4 => "C4",
            BlockKeyNote::D4 => "D4",
            BlockKeyNote::E4 => "E4",
            BlockKeyNote::F4 => "F4",
            BlockKeyNote::G4 => "G4",
            BlockKeyNote::A4 => "A4",
            BlockKeyNote::B4 => "B4",
            BlockKeyNote::C5 => "C5",
            BlockKeyNote::D5 => "D5",
            BlockKeyNote::E5 => "E5",
            BlockKeyNote::F5 => "F5",
            BlockKeyNote::G5 => "G5",
            BlockKeyNote::A5 => "A5",
            BlockKeyNote::B5 => "B5",
            BlockKeyNote::None => "",
        }
    }

    /// File name for the Salamander Grand Piano sample used as the source.
    pub fn sample_file(self) -> &'static str {
        match self {
            BlockKeyNote::C3 | BlockKeyNote::D3 | BlockKeyNote::E3 => "C3v8.flac",
            BlockKeyNote::F3 | BlockKeyNote::G3 => "F#3v8.flac",
            BlockKeyNote::A3 | BlockKeyNote::B3 => "A3v8.flac",
            BlockKeyNote::C4 | BlockKeyNote::D4 | BlockKeyNote::E4 => "C4v8.flac",
            BlockKeyNote::F4 | BlockKeyNote::G4 => "F#4v8.flac",
            BlockKeyNote::A4 | BlockKeyNote::B4 => "A4v8.flac",
            BlockKeyNote::C5 | BlockKeyNote::D5 | BlockKeyNote::E5 => "C5v8.flac",
            BlockKeyNote::F5 | BlockKeyNote::G5 => "F#5v8.flac",
            BlockKeyNote::A5 | BlockKeyNote::B5 => "A5v8.flac",
            BlockKeyNote::None => "",
        }
    }

    /// Speed ratio applied to the source sample to reach the target pitch.
    pub fn playback_speed(self) -> f32 {
        match self {
            BlockKeyNote::C3 => 1.0,
            BlockKeyNote::D3 => 1.1225,
            BlockKeyNote::E3 => 1.2599,
            BlockKeyNote::F3 => 0.9439,
            BlockKeyNote::G3 => 1.0595,
            BlockKeyNote::A3 => 1.0,
            BlockKeyNote::B3 => 0.9439,
            BlockKeyNote::C4 => 1.0,
            BlockKeyNote::D4 => 1.1225,
            BlockKeyNote::E4 => 1.2599,
            BlockKeyNote::F4 => 0.9439,
            BlockKeyNote::G4 => 1.0595,
            BlockKeyNote::A4 => 1.0,
            BlockKeyNote::B4 => 0.9439,
            BlockKeyNote::C5 => 1.0,
            BlockKeyNote::D5 => 1.1225,
            BlockKeyNote::E5 => 1.2599,
            BlockKeyNote::F5 => 0.9439,
            BlockKeyNote::G5 => 1.0595,
            BlockKeyNote::A5 => 1.0,
            BlockKeyNote::B5 => 0.9439,
            BlockKeyNote::None => 1.0,
        }
    }

    /// Convert a lowercase character to its corresponding note.
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            // Lower octave (C3-B3)
            'z' => Some(BlockKeyNote::C3),
            'x' => Some(BlockKeyNote::D3),
            'c' => Some(BlockKeyNote::E3),
            'v' => Some(BlockKeyNote::F3),
            'b' => Some(BlockKeyNote::G3),
            'n' => Some(BlockKeyNote::A3),
            'm' => Some(BlockKeyNote::B3),
            // Middle octave (C4-B4)
            'a' => Some(BlockKeyNote::C4),
            's' => Some(BlockKeyNote::D4),
            'd' => Some(BlockKeyNote::E4),
            'f' => Some(BlockKeyNote::F4),
            'g' => Some(BlockKeyNote::G4),
            'h' => Some(BlockKeyNote::A4),
            'j' => Some(BlockKeyNote::B4),
            // Upper octave (C5-B5)
            'q' => Some(BlockKeyNote::C5),
            'w' => Some(BlockKeyNote::D5),
            'e' => Some(BlockKeyNote::E5),
            'r' => Some(BlockKeyNote::F5),
            't' => Some(BlockKeyNote::G5),
            'y' => Some(BlockKeyNote::A5),
            'u' => Some(BlockKeyNote::B5),
            _ => None,
        }
    }

    /// Converts an interactive key press into a `BlockKeyNote`.
    pub fn from_key_code(key: KeyCode) -> Option<Self> {
        if let KeyCode::Char(ch) = key {
            Self::from_char(ch)
        } else {
            None
        }
    }
}

/// Audio command for the shared audio thread.
#[derive(Debug)]
enum AudioCommand {
    Play {
        sample_file: String,
        speed: f32,
    },
}

/// Shared audio sender for all block key notes.
/// Uses a single OutputStream that lives for the duration of the program.
#[cfg(feature = "interactive")]
static BLOCK_AUDIO_SENDER: OnceCellLazy<mpsc::Sender<AudioCommand>> = OnceCellLazy::new(|| {
    let (tx, rx) = mpsc::channel::<AudioCommand>();

    thread::spawn(move || {
        let Ok(stream) = OutputStreamBuilder::open_default_stream() else {
            eprintln!("[BlockKey] Failed to open audio stream");
            return;
        };
        let mixer = stream.mixer().clone();

        let mut sinks: VecDeque<Sink> = VecDeque::new();

        for cmd in rx {
            let (sample_file, speed) = match cmd {
                AudioCommand::Play { sample_file, speed } => (sample_file, speed),
            };

            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("salamander-grand-piano-in-rust")
                .join("Samples")
                .join(&sample_file);

            let source_data = std::fs::read(&path).ok();

            if let Some(source_data) = source_data {
                let cursor = Cursor::new(source_data);
                if let Ok(source) = Decoder::new(cursor) {
                    let source = source.speed(speed);
                    let sink = Sink::connect_new(&mixer);
                    sink.append(source);
                    sinks.push_back(sink);
                }
            }

            // Clean up empty sinks
            sinks.retain(|s| !s.empty());

            // Limit concurrent sinks
            if sinks.len() > 8 {
                if let Some(old) = sinks.pop_front() {
                    old.stop();
                }
            }
        }
    });

    tx
});

/// Play a note using the shared audio thread.
/// Returns true if note should be displayed in message, false otherwise.
#[cfg(feature = "interactive")]
pub fn play_note(note: BlockKeyNote) -> bool {
    // Skip sound for None notes
    if matches!(note, BlockKeyNote::None) {
        return false;
    }

    let sample_file = note.sample_file();
    let speed = note.playback_speed();
    let _ = BLOCK_AUDIO_SENDER.send(AudioCommand::Play {
        sample_file: sample_file.to_string(),
        speed,
    });
    true
}

#[cfg(not(feature = "interactive"))]
pub fn play_note(_note: BlockKeyNote) {
    // No-op when audio is disabled
}

/// Tracks currently held keys to debounce key-repeat events.
pub struct BlockKeyState {
    pressed: HashSet<char>,
}

impl BlockKeyState {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    /// Returns true if this is a fresh press (not a repeat).
    pub fn key_down(&mut self, c: char) -> bool {
        self.pressed.insert(c.to_ascii_lowercase())
    }

    /// Call on key release to allow re-triggering.
    pub fn key_up(&mut self, c: char) {
        self.pressed.remove(&c.to_ascii_lowercase());
    }
}
