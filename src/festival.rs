use crate::map::RegionMap;
use crate::season::is_butterfly_festival;
use crate::types::{Region, TileType};
use std::collections::HashMap;

pub fn update_festival_state(maps: &mut HashMap<Region, RegionMap>, day: u32) {
    if is_butterfly_festival(day) {
        if let Some(square) = maps.get_mut(&Region::Square) {
            square.set(2, 2, TileType::Wonder);
        }
    } else if let Some(square) = maps.get_mut(&Region::Square) {
        if let Some(tile) = get_square_wonder_tile() {
            if matches!(square.get(2, 2), Some(TileType::Wonder)) {
                square.set(2, 2, tile);
            }
        }
    }
}

fn get_square_wonder_tile() -> Option<TileType> {
    Some(TileType::Grass)
}

pub fn handle_wonder_message() -> &'static str {
    "That is so beautiful. Let's enjoy it together in the game."
}
