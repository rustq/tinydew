use crate::types::Weather;

pub fn roll_weather(day: u32, seed: u64) -> Weather {
    if day == 1 {
        return Weather::Sunny;
    }

    let festival_day = 28;
    if day == festival_day {
        return Weather::Sunny;
    }

    let hash = seed.wrapping_mul(1103515245).wrapping_add(12345);
    let value = (hash % 100) as u32;

    if value < 50 {
        Weather::Sunny
    } else if value < 80 {
        Weather::Cloudy
    } else {
        Weather::Rainy
    }
}
