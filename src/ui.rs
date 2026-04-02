use crate::state::GameState;

pub fn render_status(state: &GameState) -> String {
    let mut lines = Vec::new();

    // Top line
    let weather_icon = state.weather.icon(state.time.is_night());
    lines.push(format!(
        "tinydew day {} {} {}",
        state.day,
        weather_icon,
        state.time.format()
    ));

    // Map block
    let map = state.get_map(state.player.location);
    for (y, row) in map.iter().enumerate() {
        let mut line = String::new();
        for (x, tile) in row.iter().enumerate() {
            if state.player.location == state.player.location
                && state.player.x == x
                && state.player.y == y
            {
                line.push_str("🧑");
            } else if state.guest.location == state.player.location
                && state.guest.x == x
                && state.guest.y == y
            {
                line.push_str("👧");
            } else {
                line.push_str(tile.emoji());
            }
        }
        lines.push(line);
    }

    // Inventory items (only non-empty)
    let items = state.inventory.format_items();
    for item in items {
        lines.push(item);
    }

    // Money line
    lines.push(format!("Money: 💰 ${}", state.inventory.money));

    // Bottom message
    lines.push(state.message.clone());

    lines.join("\n")
}
