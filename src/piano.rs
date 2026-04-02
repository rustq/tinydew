use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PianoNote {
    C3, D3, E3, F3, G3, A3, B3,
    C4, D4, E4, F4, G4, A4, B4,
    C5, D5, E5, F5, G5, A5, B5,
}

impl PianoNote {
    /// Parse from CLI string like "C4", "D3", "A5"
    pub fn from_name(s: &str) -> Option<PianoNote> {
        match s {
            "C3" => Some(PianoNote::C3),
            "D3" => Some(PianoNote::D3),
            "E3" => Some(PianoNote::E3),
            "F3" => Some(PianoNote::F3),
            "G3" => Some(PianoNote::G3),
            "A3" => Some(PianoNote::A3),
            "B3" => Some(PianoNote::B3),
            "C4" => Some(PianoNote::C4),
            "D4" => Some(PianoNote::D4),
            "E4" => Some(PianoNote::E4),
            "F4" => Some(PianoNote::F4),
            "G4" => Some(PianoNote::G4),
            "A4" => Some(PianoNote::A4),
            "B4" => Some(PianoNote::B4),
            "C5" => Some(PianoNote::C5),
            "D5" => Some(PianoNote::D5),
            "E5" => Some(PianoNote::E5),
            "F5" => Some(PianoNote::F5),
            "G5" => Some(PianoNote::G5),
            "A5" => Some(PianoNote::A5),
            "B5" => Some(PianoNote::B5),
            _ => None,
        }
    }

    /// Map keyboard key (interactive mode) to note
    pub fn from_key(c: char) -> Option<PianoNote> {
        match c.to_ascii_lowercase() {
            'z' => Some(PianoNote::C3),
            'x' => Some(PianoNote::D3),
            'c' => Some(PianoNote::E3),
            'v' => Some(PianoNote::F3),
            'b' => Some(PianoNote::G3),
            'n' => Some(PianoNote::A3),
            'm' => Some(PianoNote::B3),
            'a' => Some(PianoNote::C4),
            's' => Some(PianoNote::D4),
            'd' => Some(PianoNote::E4),
            'f' => Some(PianoNote::F4),
            'g' => Some(PianoNote::G4),
            'h' => Some(PianoNote::A4),
            'j' => Some(PianoNote::B4),
            'q' => Some(PianoNote::C5),
            'w' => Some(PianoNote::D5),
            'e' => Some(PianoNote::E5),
            'r' => Some(PianoNote::F5),
            't' => Some(PianoNote::G5),
            'y' => Some(PianoNote::A5),
            'u' => Some(PianoNote::B5),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PianoNote::C3 => "C3", PianoNote::D3 => "D3", PianoNote::E3 => "E3",
            PianoNote::F3 => "F3", PianoNote::G3 => "G3", PianoNote::A3 => "A3",
            PianoNote::B3 => "B3",
            PianoNote::C4 => "C4", PianoNote::D4 => "D4", PianoNote::E4 => "E4",
            PianoNote::F4 => "F4", PianoNote::G4 => "G4", PianoNote::A4 => "A4",
            PianoNote::B4 => "B4",
            PianoNote::C5 => "C5", PianoNote::D5 => "D5", PianoNote::E5 => "E5",
            PianoNote::F5 => "F5", PianoNote::G5 => "G5", PianoNote::A5 => "A5",
            PianoNote::B5 => "B5",
        }
    }

    /// Sample file and speed ratio for pitch-shifting
    pub fn sample_info(&self) -> (&'static str, f32) {
        match self {
            PianoNote::C3 => ("files/C3v8.flac", 1.0),
            PianoNote::D3 => ("files/C3v8.flac", 1.1225),
            PianoNote::E3 => ("files/C3v8.flac", 1.2599),
            PianoNote::F3 => ("files/F#3v8.flac", 1.0),
            PianoNote::G3 => ("files/F#3v8.flac", 1.0595),
            PianoNote::A3 => ("files/A3v8.flac", 1.0),
            PianoNote::B3 => ("files/A3v8.flac", 1.0595),
            PianoNote::C4 => ("files/C4v8.flac", 1.0),
            PianoNote::D4 => ("files/C4v8.flac", 1.1225),
            PianoNote::E4 => ("files/C4v8.flac", 1.2599),
            PianoNote::F4 => ("files/F#4v8.flac", 1.0),
            PianoNote::G4 => ("files/F#4v8.flac", 1.0595),
            PianoNote::A4 => ("files/A4v8.flac", 1.0),
            PianoNote::B4 => ("files/A4v8.flac", 1.0595),
            PianoNote::C5 => ("files/C5v8.flac", 1.0),
            PianoNote::D5 => ("files/C5v8.flac", 1.1225),
            PianoNote::E5 => ("files/C5v8.flac", 1.2599),
            PianoNote::F5 => ("files/F#5v8.flac", 1.0),
            PianoNote::G5 => ("files/F#5v8.flac", 1.0595),
            PianoNote::A5 => ("files/A5v8.flac", 1.0),
            PianoNote::B5 => ("files/A5v8.flac", 1.0595),
        }
    }
}

#[cfg(feature = "interactive")]
pub mod audio {
    use super::PianoNote;
    use rodio::{Sink, OutputStreamBuilder};
    use std::io::BufReader;
    use std::fs::File;

    pub fn play_note(note: PianoNote) {
        let (sample_path, speed) = note.sample_info();
        std::thread::spawn(move || {
            let stream = match OutputStreamBuilder::open_default_stream() {
                Ok(s) => s,
                Err(_) => return,
            };
            let sink = Sink::connect_new(&stream.mixer());
            let file = match File::open(sample_path) {
                Ok(f) => f,
                Err(_) => return,
            };
            let source = match rodio::Decoder::new(BufReader::new(file)) {
                Ok(s) => s,
                Err(_) => return,
            };
            sink.set_speed(speed);
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}
