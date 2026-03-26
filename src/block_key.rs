use std::collections::HashSet;

use crossterm::event::KeyCode;

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

/// Fire-and-forget: spawn a thread that plays the note.
/// Audio initialization failures are silently ignored.
#[cfg(feature = "interactive")]
pub fn play_note(note: BlockKeyNote) {
    use std::f32::consts::PI;
    use std::io::Cursor;
    use std::thread;

    use rodio::{Decoder, OutputStreamBuilder, Sink};

    thread::spawn(move || {
        let Ok(stream) = OutputStreamBuilder::open_default_stream() else {
            return;
        };
        let sink = Sink::connect_new(&stream.mixer().clone());

        let wav_data = generate_sine_wav(note.frequency());
        let cursor = Cursor::new(wav_data);
        if let Ok(source) = rodio::Decoder::new(cursor) {
            sink.append(source);
        }
    });
}

#[cfg(not(feature = "interactive"))]
pub fn play_note(_note: BlockKeyNote) {
    // No-op when audio is disabled
}

/// Generate a 500ms sine-wave WAV at the given frequency, with 0.3 gain.
#[cfg(feature = "interactive")]
fn generate_sine_wav(freq: f32) -> Vec<u8> {
    use std::f32::consts::PI;

    let sample_rate: u32 = 44100;
    let duration_secs: f32 = 0.5;
    let gain: f32 = 0.3;
    let num_samples = (sample_rate as f32 * duration_secs) as usize;

    let attack_samples = (sample_rate as f32 * 0.01) as usize;
    let release_samples = (sample_rate as f32 * 0.01) as usize;

    let mut samples: Vec<i16> = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let raw = (2.0 * PI * freq * t).sin();

        let envelope = if i < attack_samples {
            i as f32 / attack_samples as f32
        } else if i > num_samples - release_samples {
            (num_samples - i) as f32 / release_samples as f32
        } else {
            1.0
        };

        let sample = (raw * gain * envelope * i16::MAX as f32) as i16;
        samples.push(sample);
    }

    let data_size = (num_samples * 2) as u32;
    let file_size = 36 + data_size;
    let mut buf: Vec<u8> = Vec::with_capacity(44 + data_size as usize);

    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());
    for s in &samples {
        buf.extend_from_slice(&s.to_le_bytes());
    }

    buf
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
