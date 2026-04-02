#[cfg(feature = "interactive")]
pub mod audio {
    use rodio::{Sink, OutputStreamBuilder};
    use rodio::source::SineWave;
    use std::time::Duration;

    const FREQUENCIES: [(char, f32); 10] = [
        ('q', 261.63), // C4
        ('w', 293.66), // D4
        ('e', 329.63), // E4
        ('r', 349.23), // F4
        ('t', 392.00), // G4
        ('y', 440.00), // A4
        ('u', 493.88), // B4
        ('i', 523.25), // C5
        ('o', 587.33), // D5
        ('p', 659.25), // E5
    ];

    pub fn freq_for_key(c: char) -> Option<f32> {
        let lower = c.to_ascii_lowercase();
        FREQUENCIES.iter().find(|(k, _)| *k == lower).map(|(_, f)| *f)
    }

    pub fn play_block_key(freq: f32) {
        std::thread::spawn(move || {
            let stream = match OutputStreamBuilder::open_default_stream() {
                Ok(s) => s,
                Err(_) => return,
            };
            let sink = Sink::connect_new(&stream.mixer());
            let source = SineWave::new(freq);
            sink.set_volume(0.3);
            sink.append(source);
            std::thread::sleep(Duration::from_millis(500));
            sink.stop();
        });
    }
}
