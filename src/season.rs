use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub fn from_day(day: u32) -> Self {
        let day_in_year = ((day - 1) % 112) + 1;
        if day_in_year <= 28 {
            Season::Spring
        } else if day_in_year <= 56 {
            Season::Summer
        } else if day_in_year <= 84 {
            Season::Fall
        } else {
            Season::Winter
        }
    }
}

pub fn is_butterfly_festival(day: u32) -> bool {
    let day_in_year = ((day - 1) % 112) + 1;
    Season::from_day(day) == Season::Spring && day_in_year == 28
}

pub fn get_festival_message(day: u32) -> Option<String> {
    if is_butterfly_festival(day) {
        Some("Today is Butterfly Festival, enjoy it!".to_string())
    } else {
        None
    }
}
