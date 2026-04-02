use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

impl Weather {
    pub fn icon(&self, is_night: bool) -> &'static str {
        if is_night {
            return "🌙";
        }
        match self {
            Weather::Sunny => "☀️",
            Weather::Cloudy => "⛅",
            Weather::Rainy => "🌧",
        }
    }
}

pub fn roll_weather(day: u32) -> Weather {
    if day == 1 || day == 28 {
        return Weather::Sunny;
    }
    // Deterministic seed-based roll favoring Sunny > Cloudy > Rainy
    let seed = day.wrapping_mul(2654435761);
    let val = seed % 100;
    if val < 50 {
        Weather::Sunny
    } else if val < 80 {
        Weather::Cloudy
    } else {
        Weather::Rainy
    }
}
