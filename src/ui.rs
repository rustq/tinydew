use crate::state::GameState;

pub fn render_status(state: &GameState) -> String {
    let mut output = String::new();

    let weather_emoji = state.weather.emoji(state.time.is_night());
    output.push_str(&format!(
        "tinydew day {} {} {}\n",
        state.time.day,
        weather_emoji,
        state.time.format_time()
    ));

    output.push('\n');

    if let Some(map) = state.get_current_map() {
        for y in 0..map.height {
            for x in 0..map.width {
                if state.player.x == x && state.player.y == y {
                    output.push('🧑');
                } else {
                    let tile =
                        if let Some(crop_data) = state.crops.get(&(state.player.region, x, y)) {
                            crate::types::TileType::Crop(crop_data.crop_type, crop_data.is_mature())
                        } else {
                            map.get(x, y).unwrap_or(crate::types::TileType::Boundary)
                        };
                    output.push_str(tile.emoji());
                }
            }
            output.push('\n');
        }
    }

    output.push('\n');

    if state.inventory.seeds > 0 {
        output.push_str(&format!("seeds: 🫙 x{}\n", state.inventory.seeds));
    }

    if !state.inventory.produce.is_empty() {
        for (emoji, count) in &state.inventory.produce {
            output.push_str(&format!("{} x{}\n", emoji, count));
        }
    }

    if !state.inventory.forage.is_empty() {
        for (emoji, count) in &state.inventory.forage {
            output.push_str(&format!("{} x{}\n", emoji, count));
        }
    }

    if !state.inventory.fish.is_empty() {
        for (emoji, count) in &state.inventory.fish {
            output.push_str(&format!("{} x{}\n", emoji, count));
        }
    }

    if state.inventory.seeds > 0
        || !state.inventory.produce.is_empty()
        || !state.inventory.forage.is_empty()
        || !state.inventory.fish.is_empty()
    {
        output.push('\n');
    }

    output.push_str(&format!("Money: 💰 ${}\n", state.money));
    output.push('\n');

    output.push_str(&state.bottom_message);

    output
}
