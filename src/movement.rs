use crate::entity::Direction;
use crate::map::{Region, TileType};
use crate::state::GameState;

pub fn move_player(state: &mut GameState, dir: Direction) -> String {
    state.player.direction = dir;
    let (dx, dy) = dir.delta();
    let nx = state.player.x as i32 + dx;
    let ny = state.player.y as i32 + dy;

    let region = state.player.location;
    let map = state.get_map(region);
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    if nx < 0 || ny < 0 || nx >= width || ny >= height {
        return "Can't go there.".to_string();
    }

    let (ux, uy) = (nx as usize, ny as usize);

    // Snapshot tile info before any mutable borrow
    let tile_walkable = map[uy][ux].is_walkable();
    let tile_mature = map[uy][ux].is_mature_crop();
    let tile_is_wonder = matches!(map[uy][ux], TileType::Wonder);
    let tile_is_piano = matches!(map[uy][ux], TileType::Piano);
    let tile_clone = map[uy][ux].clone();

    // Check gate transitions
    if tile_clone.is_path() {
        if let Some(msg) = check_transition(state, &tile_clone) {
            return msg;
        }
    }

    // Check walkability
    if !tile_walkable {
        if tile_mature {
            return "Crop is ready to harvest! Harvest it first.".to_string();
        }
        if tile_is_wonder {
            return "That is so beautiful. Let human enjoy it together in interactive mode.".to_string();
        }
        if tile_is_piano {
            return "A beautiful old piano. It hums quietly on the farm.".to_string();
        }
        return "Can't go there.".to_string();
    }

    // Check if guest is at that position in same region
    if state.guest.location == state.player.location
        && state.guest.x == ux
        && state.guest.y == uy
    {
        return "Someone is there.".to_string();
    }

    state.player.x = ux;
    state.player.y = uy;
    state.tick_time();

    format!("Moved {}.", dir_name(dir))
}

fn dir_name(dir: Direction) -> &'static str {
    match dir {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn check_transition(state: &mut GameState, tile: &TileType) -> Option<String> {
    let region = state.player.location;

    match (region, tile) {
        (Region::Farm, TileType::PathEast) => {
            // Farm -> EastPath
            state.player.x = 1;
            state.player.y = 2;
            state.player.location = Region::EastPath;
            state.player.direction = Direction::Right;
            state.tick_time();
            Some("Entered East Path.".to_string())
        }
        (Region::EastPath, TileType::PathFarm) => {
            // EastPath -> Farm
            state.player.x = 6;
            state.player.y = 5;
            state.player.location = Region::Farm;
            state.player.direction = Direction::Left;
            state.tick_time();
            Some("Returned to Farm.".to_string())
        }
        (Region::EastPath, TileType::PathSquare) => {
            // EastPath -> Square
            state.player.x = 4;
            state.player.y = 3;
            state.player.location = Region::Square;
            state.player.direction = Direction::Up;
            state.tick_time();
            Some("Entered the Square.".to_string())
        }
        (Region::Square, TileType::PathSquare) => {
            // Square -> EastPath
            state.player.x = 5;
            state.player.y = 1;
            state.player.location = Region::EastPath;
            state.player.direction = Direction::Down;
            state.tick_time();
            Some("Left the Square.".to_string())
        }
        (Region::EastPath, TileType::PathSouthRiver) => {
            // EastPath -> SouthRiver
            state.player.x = 2;
            state.player.y = 1;
            state.player.location = Region::SouthRiver;
            state.player.direction = Direction::Down;
            state.tick_time();
            Some("Arrived at the South River.".to_string())
        }
        (Region::SouthRiver, TileType::PathSouthRiverGate) => {
            // SouthRiver -> EastPath
            state.player.x = 2;
            state.player.y = 2;
            state.player.location = Region::EastPath;
            state.player.direction = Direction::Up;
            state.tick_time();
            Some("Left the South River.".to_string())
        }
        _ => None,
    }
}
