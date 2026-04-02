use crate::map::TileType;
use crate::state::GameState;
use crate::weather::Weather;

pub fn grow_crops(state: &mut GameState) {
    let rainy = state.weather == Weather::Rainy;

    for region_key in &["Farm", "EastPath", "Square", "SouthRiver"] {
        let region_map = state.maps.get_mut(*region_key).unwrap();
        for row in region_map.iter_mut() {
            for tile in row.iter_mut() {
                if let TileType::Crop {
                    days_grown,
                    watered_today,
                    ..
                } = tile
                {
                    // Rainy weather auto-waters
                    if rainy {
                        *watered_today = true;
                    }
                    // Grow if watered
                    if *watered_today {
                        *days_grown += 1;
                    }
                    // Reset watered state for new day
                    *watered_today = false;
                }
            }
        }
    }
}

pub fn reset_river_bubbles(state: &mut GameState) {
    let map = state.maps.get_mut("SouthRiver").unwrap();
    for row in map.iter_mut() {
        for tile in row.iter_mut() {
            if matches!(tile, TileType::River) {
                // Randomly place some bubbles
                // Simple deterministic approach: use day to seed
            }
        }
    }
    // Place a few bubbles based on day
    let day = state.day;
    let bubble_x = ((day * 7) % 13) as usize;
    if map[2][bubble_x] == TileType::River {
        map[2][bubble_x] = TileType::RiverBubble;
    }
    let bubble_x2 = ((day * 11 + 3) % 13) as usize;
    if map[3][bubble_x2] == TileType::River {
        map[3][bubble_x2] = TileType::RiverBubble;
    }
}
