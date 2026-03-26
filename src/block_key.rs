use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crossterm::event::KeyCode;
use once_cell::sync::Lazy as OnceCellLazy;
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};

/// Musical notes mapped to keyboard keys (C Major scale, C4–E5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKeyNote {
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
}

impl BlockKeyNote {
    /// Human-readable name displayed in the bottom message.
    pub fn display_name(self) -> &'static str {
        match self {
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
        }
    }

    /// Frequency in Hz for this note.
    pub fn frequency(self) -> f32 {
        match self {
            BlockKeyNote::C4 => 261.63,
            BlockKeyNote::D4 => 293.66,
            BlockKeyNote::E4 => 329.63,
            BlockKeyNote::F4 => 349.23,
            BlockKeyNote::G4 => 392.00,
            BlockKeyNote::A4 => 440.00,
            BlockKeyNote::B4 => 493.88,
            BlockKeyNote::C5 => 523.25,
            BlockKeyNote::D5 => 587.33,
            BlockKeyNote::E5 => 659.25,
        }
    }

    /// Convert a lowercase character to its corresponding note.
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'q' => Some(Self::C4),
            'w' => Some(Self::D4),
            'e' => Some(Self::E4),
            'r' => Some(Self::F4),
            't' => Some(Self::G4),
            'y' => Some(Self::A4),
            'u' => Some(Self::B4),
            'i' => Some(Self::C5),
            'o' => Some(Self::D5),
            'p' => Some(Self::E5),
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

/// Map a BlockKeyNote to its corresponding sample file and playback speed.
fn sample_info(note: BlockKeyNote) -> (&'static str, f32) {
    match note {
        BlockKeyNote::C4 => ("C4v8.flac", 1.0),
        BlockKeyNote::D4 => ("C4v8.flac", 1.1225),
        BlockKeyNote::E4 => ("C4v8.flac", 1.2599),
        BlockKeyNote::F4 => ("C4v8.flac", 0.9439),
        BlockKeyNote::G4 => ("C4v8.flac", 1.0595),
        BlockKeyNote::A4 => ("A4v8.flac", 1.0),
        BlockKeyNote::B4 => ("A4v8.flac", 0.9439),
        BlockKeyNote::C5 => ("C5v8.flac", 1.0),
        BlockKeyNote::D5 => ("C5v8.flac", 1.1225),
        BlockKeyNote::E5 => ("C5v8.flac", 1.2599),
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

            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                if let Ok(source) = Decoder::new(reader) {
                    let source = source.speed(speed);
                    let sink = Sink::connect_new(&mixer);
                    sink.append(source);
                    sinks.push_back(sink);
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

/// Play a note using the shared audio thread.
#[cfg(feature = "interactive")]
pub fn play_note(note: BlockKeyNote) {
    let (sample_file, speed) = sample_info(note);
    let _ = BLOCK_AUDIO_SENDER.send(AudioCommand::Play {
        sample_file: sample_file.to_string(),
        speed,
    });
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
