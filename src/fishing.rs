use crate::entity::Direction;
use crate::map::{Region, TileType};
use crate::state::GameState;
use rand::Rng;

pub fn fish(state: &mut GameState, dir: Option<Direction>) -> String {
    if state.player.location != Region::SouthRiver {
        return "No river nearby to fish.".to_string();
    }

    let d = dir.unwrap_or(state.player.direction);
    let (dx, dy) = d.delta();
    let nx = state.player.x as i32 + dx;
    let ny = state.player.y as i32 + dy;

    let map = state.get_map(state.player.location);
    if ny < 0 || nx < 0 || ny >= map.len() as i32 || nx >= map[0].len() as i32 {
        return "Can't fish there.".to_string();
    }

    let (ux, uy) = (nx as usize, ny as usize);
    let tile = &map[uy][ux];

    if !matches!(tile, TileType::River | TileType::RiverBubble) {
        return "Can't fish there.".to_string();
    }

    let is_bubble = matches!(tile, TileType::RiverBubble);

    // Reset bubble to river after fishing
    if is_bubble {
        state.get_map_mut(Region::SouthRiver)[uy][ux] = TileType::River;
    }

    state.tick_time();

    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(0..100);

    if is_bubble {
        // Bubble = guaranteed catch, rare chance higher
        if roll < 30 {
            state.inventory.rare_fish += 1;
            "Caught a rare fish! 🐠".to_string()
        } else {
            state.inventory.fish += 1;
            "Caught a fish! 🐟".to_string()
        }
    } else if roll < 40 {
        if roll < 10 {
            state.inventory.rare_fish += 1;
            "Caught a rare fish! 🐠".to_string()
        } else {
            state.inventory.fish += 1;
            "Caught a fish! 🐟".to_string()
        }
    } else {
        "No bite...".to_string()
    }
}
