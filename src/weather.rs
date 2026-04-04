use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

impl Weather {
    pub fn emoji(self) -> &'static str {
        match self {
            Weather::Sunny => "☀️",
            Weather::Cloudy => "⛅",
            Weather::Rainy => "🌧",
        }
    }
}

/// Roll weather for a given day. Day 1 and Spring Day 28 are forced Sunny.
pub fn roll_weather(day: u32) -> Weather {
    if day == 1 || day == 28 {
        return Weather::Sunny;
    }

    // Deterministic seed-based: favor Sunny > Cloudy > Rainy
    let hash = simple_hash(day);
    let roll = hash % 100;
    if roll < 50 {
        Weather::Sunny
    } else if roll < 80 {
        Weather::Cloudy
    } else {
        Weather::Rainy
    }
}

fn simple_hash(day: u32) -> u32 {
    let mut x = day.wrapping_mul(2654435761);
    x ^= x >> 16;
    x = x.wrapping_mul(0x45d9f3b);
    x ^= x >> 16;
    x
}
