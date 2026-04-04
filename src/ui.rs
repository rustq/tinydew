use crate::state::GameState;

pub fn render_status(state: &GameState) -> String {
    let mut output = String::new();

    // Header: tinydew day <day> <weather_icon> <time>
    output.push_str(&format!(
        "tinydew day {} {} {}\n",
        state.day,
        state.weather_icon(),
        state.time_string()
    ));

    // Map block
    let map = state.current_map();
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if x == state.player.x && y == state.player.y {
                output.push('🧑');
            } else {
                output.push_str(tile.emoji());
            }
        }
        output.push('\n');
    }

    // Inventory items (non-empty only, no header)
    if state.inventory.seeds > 0 {
        output.push_str(&format!("seeds: 🫙 x{}\n", state.inventory.seeds));
    }

    let items = state.inventory.display_items();
    for (emoji, count) in items {
        output.push_str(&format!("{emoji}: x{count}\n"));
    }

    // Money line
    output.push_str(&format!("Money: 💰 ${}\n", state.money));

    // Bottom message
    output.push_str(&state.message);
    output.push('\n');

    output
}
