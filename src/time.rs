use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameTime {
    pub day: u32,
    pub minutes: u32,
}

impl GameTime {
    pub fn new(day: u32, hour: u32, minute: u32) -> Self {
        Self {
            day,
            minutes: hour * 60 + minute,
        }
    }

    pub fn start_of_day(day: u32) -> Self {
        Self::new(day, 6, 0)
    }

    pub fn hour(&self) -> u32 {
        self.minutes / 60
    }

    pub fn minute(&self) -> u32 {
        self.minutes % 60
    }

    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}", self.hour(), self.minute())
    }

    pub fn is_night(&self) -> bool {
        self.hour() >= 20 || self.hour() < 6
    }

    pub fn advance(&mut self, minutes: u32) {
        self.minutes += minutes;
        while self.minutes >= 24 * 60 {
            self.minutes -= 24 * 60;
            self.day += 1;
        }
    }

    pub fn next_day(&mut self) {
        self.day += 1;
        self.minutes = 6 * 60;
    }
}
