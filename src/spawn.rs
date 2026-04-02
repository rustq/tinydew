use crate::map::{Region, TileType};
use crate::state::GameState;

/// Protected tiles that spawns cannot overwrite
fn is_protected(region: Region, x: usize, y: usize) -> bool {
    match region {
        Region::Farm => {
            // House (2,2), Piano (4,2), wake-up (3,3)
            (x == 2 && y == 2) || (x == 4 && y == 2) || (x == 3 && y == 3)
        }
        _ => false,
    }
}

fn has_blocker(map: &[Vec<TileType>]) -> bool {
    for row in map {
        for tile in row {
            if matches!(tile, TileType::Mushroom | TileType::Crop { .. })
                && !tile.is_walkable()
            {
                // Flower (mature crop) or mushroom counts as a blocker
                if matches!(tile, TileType::Mushroom) {
                    return true;
                }
                if tile.is_mature_crop() {
                    return true;
                }
            }
        }
    }
    false
}

pub fn nightly_spawns(state: &mut GameState) {
    let day = state.day;

    // Try spawn on Farm
    spawn_in_region(state, Region::Farm, day);
    // Try spawn on EastPath
    spawn_in_region(state, Region::EastPath, day);
    // Try spawn on Square
    spawn_in_region(state, Region::Square, day);
}

fn spawn_in_region(state: &mut GameState, region: Region, day: u32) {
    let key = GameState::region_key(region);
    let map = state.maps.get(key).unwrap();

    if has_blocker(map) {
        return;
    }

    // Find empty grass tiles
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, TileType::Grass) && !is_protected(region, x, y) {
                candidates.push((x, y));
            }
        }
    }

    if candidates.is_empty() {
        return;
    }

    // Deterministic pick based on day and region
    let seed = day.wrapping_mul(31).wrapping_add(region as u32 * 17);
    let idx = (seed as usize) % candidates.len();
    let (sx, sy) = candidates[idx];

    let map = state.maps.get_mut(key).unwrap();
    // Mushroom spawn on Farm/EastPath, flower on Square
    if region == Region::Square {
        // Decorative flower spawn as mature crop
        map[sy][sx] = TileType::Crop {
            crop_type: crate::map::CropType::Flower,
            days_grown: 2,
            watered_today: false,
        };
    } else {
        map[sy][sx] = TileType::Mushroom;
    }
}
