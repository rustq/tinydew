use crate::state::GameState;
use crate::types::*;

pub fn render_status(state: &GameState) {
    print!("{}", format_status(state));
}

pub fn format_status(state: &GameState) -> String {
    let mut out = String::new();

    // Top line: tinydew day <day> <weather_icon> <time>
    let night = is_night(state.time_minutes);
    let weather_icon = state.weather.icon(night);
    let time_str = format_time(state.time_minutes);
    out.push_str(&format!("tinydew day {} {} {}\n", state.day, weather_icon, time_str));

    // Map
    let map = state.maps.get(&state.player.region);
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if state.player.region == state.player.region
                && state.player.x == x
                && state.player.y == y
            {
                out.push_str("\u{1f9d1}"); // 🧑
            } else {
                out.push_str(tile.emoji());
            }
        }
        out.push('\n');
    }

    // Inventory (non-empty items only, no header)
    let inv = &state.inventory;
    if inv.seeds > 0 {
        out.push_str(&format!("\u{1fad9} x{}\n", inv.seeds)); // 🫙
    }
    if inv.carrots > 0 {
        out.push_str(&format!("\u{1f955} x{}\n", inv.carrots)); // 🥕
    }
    if inv.strawberries > 0 {
        out.push_str(&format!("\u{1f353} x{}\n", inv.strawberries)); // 🍓
    }
    if inv.cauliflowers > 0 {
        out.push_str(&format!("\u{1f966} x{}\n", inv.cauliflowers)); // 🥦
    }
    if inv.mushrooms > 0 {
        out.push_str(&format!("\u{1f344} x{}\n", inv.mushrooms)); // 🍄
    }
    if inv.flowers > 0 {
        out.push_str(&format!("\u{1f33a} x{}\n", inv.flowers)); // 🌺
    }
    if inv.common_fish > 0 {
        out.push_str(&format!("\u{1f41f} x{}\n", inv.common_fish)); // 🐟
    }
    if inv.rare_fish > 0 {
        out.push_str(&format!("\u{1f420} x{}\n", inv.rare_fish)); // 🐠
    }

    // Money
    out.push_str(&format!("Money: \u{1f4b0} ${}\n", state.money)); // 💰

    // Bottom message
    out.push_str(&bottom_message(state));
    out.push('\n');

    out
}

fn bottom_message(state: &GameState) -> String {
    // Festival override
    if state.season == Season::Spring && state.day == 28 {
        return "Today is Butterfly Festival, enjoy it!".to_string();
    }

    let m = state.time_minutes % 1440;
    if m < 360 {
        // 00:00 - 05:59
        "It's very late. You should get some sleep.".to_string()
    } else if m < 720 {
        // 06:00 - 11:59
        match state.weather {
            Weather::Sunny => "Good morning! The sun is shining.".to_string(),
            Weather::Cloudy => "Good morning! It's a bit cloudy.".to_string(),
            Weather::Rainy => "Good morning! It's raining outside.".to_string(),
        }
    } else if m < 1080 {
        // 12:00 - 17:59
        match state.weather {
            Weather::Sunny => "Good afternoon! Beautiful sunny day.".to_string(),
            Weather::Cloudy => "Good afternoon! Clouds are gathering.".to_string(),
            Weather::Rainy => "Good afternoon! The rain continues.".to_string(),
        }
    } else {
        // 18:00 - 23:59
        "The stars are out. Consider getting some rest.".to_string()
    }
}
