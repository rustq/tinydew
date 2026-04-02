use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub hour: u32,
    pub minute: u32,
}

impl WorldTime {
    pub fn new(hour: u32, minute: u32) -> Self {
        Self { hour, minute }
    }

    pub fn tick(&mut self) {
        self.minute += 5;
        if self.minute >= 60 {
            self.minute -= 60;
            self.hour += 1;
        }
        if self.hour >= 24 {
            self.hour = 0;
        }
    }

    pub fn is_night(&self) -> bool {
        self.hour >= 20 || self.hour < 6
    }

    pub fn format(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
}
