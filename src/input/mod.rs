use crossterm::event::Event;
use crossterm::terminal::enable_raw_mode;
use std::error::Error;
use std::time::Duration;

pub struct InputManager {
    raw_mode_enabled: bool,
}

impl InputManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        Ok(Self {
            raw_mode_enabled: true,
        })
    }

    pub fn poll(&self) -> Result<Option<Event>, Box<dyn Error>> {
        if crossterm::event::poll(Duration::from_millis(100))? {
            Ok(Some(crossterm::event::read()?))
        } else {
            Ok(None)
        }
    }
}

impl Drop for InputManager {
    fn drop(&mut self) {
        if self.raw_mode_enabled {
            let _ = crossterm::terminal::disable_raw_mode();
        }
    }
}
