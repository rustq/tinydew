use crate::map::TileType;
use crate::state::GameState;

pub fn check_festival(state: &mut GameState) {
    if state.day == 28 {
        // Butterfly Festival: place Wonder at Square (2,2)
        let map = state.maps.get_mut("Square").unwrap();
        map[2][2] = TileType::Wonder;
        state.message = "Today is Butterfly Festival, enjoy it!".to_string();
    } else {
        // Clean up wonder if past festival
        let map = state.maps.get_mut("Square").unwrap();
        if matches!(map[2][2], TileType::Wonder) {
            map[2][2] = TileType::Grass;
        }
    }
}

pub fn is_festival_day(day: u32) -> bool {
    day == 28
}
