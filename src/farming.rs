use crate::entity::Direction;
use crate::map::{CropType, Region, TileType};
use crate::state::GameState;
use rand::Rng;

fn target_pos(state: &GameState, dir: Option<Direction>) -> (usize, usize) {
    let d = dir.unwrap_or(state.player.direction);
    let (dx, dy) = d.delta();
    let nx = state.player.x as i32 + dx;
    let ny = state.player.y as i32 + dy;
    (nx as usize, ny as usize)
}

fn in_bounds(map: &[Vec<TileType>], x: usize, y: usize) -> bool {
    y < map.len() && x < map[0].len()
}

pub fn clear(state: &mut GameState, dir: Option<Direction>) -> String {
    if state.player.location != Region::Farm {
        return "Can't clear here.".to_string();
    }
    let (x, y) = target_pos(state, dir);
    let map = state.get_map(state.player.location);
    if !in_bounds(map, x, y) {
        return "Nothing to clear there.".to_string();
    }
    let tile = &map[y][x];
    match tile {
        TileType::Soil => {
            state.get_map_mut(state.player.location)[y][x] = TileType::Grass;
            state.tick_time();
            "Cleared the soil.".to_string()
        }
        TileType::Mushroom => {
            state.get_map_mut(state.player.location)[y][x] = TileType::Grass;
            state.inventory.mushrooms += 1;
            state.tick_time();
            "Collected a mushroom! 🍄".to_string()
        }
        _ => "Nothing to clear there.".to_string(),
    }
}

pub fn plant(state: &mut GameState, dir: Option<Direction>) -> String {
    if state.player.location != Region::Farm {
        return "Can't plant here.".to_string();
    }
    if state.inventory.seeds == 0 {
        return "No seeds to plant.".to_string();
    }
    let (x, y) = target_pos(state, dir);
    let map = state.get_map(state.player.location);
    if !in_bounds(map, x, y) {
        return "Can't plant there.".to_string();
    }
    let tile = &map[y][x];
    if !matches!(tile, TileType::Grass) {
        return "Can't plant there.".to_string();
    }

    let mut rng = rand::thread_rng();
    let crop_type = match rng.gen_range(0..4) {
        0 => CropType::Carrot,
        1 => CropType::Strawberry,
        2 => CropType::Cauliflower,
        _ => CropType::Flower,
    };

    state.get_map_mut(state.player.location)[y][x] = TileType::Crop {
        crop_type,
        days_grown: 0,
        watered_today: false,
    };
    state.inventory.seeds -= 1;
    state.tick_time();
    format!("Planted a {} seed! 🌱", crop_type_name(crop_type))
}

pub fn water(state: &mut GameState, dir: Option<Direction>) -> String {
    let (x, y) = target_pos(state, dir);
    let map = state.get_map(state.player.location);
    if !in_bounds(map, x, y) {
        return "Nothing to water there.".to_string();
    }
    let tile = &map[y][x];
    if let TileType::Crop { .. } = tile {
        if let TileType::Crop {
            crop_type,
            days_grown,
            ..
        } = &map[y][x]
        {
            let ct = *crop_type;
            let dg = *days_grown;
            state.get_map_mut(state.player.location)[y][x] = TileType::Crop {
                crop_type: ct,
                days_grown: dg,
                watered_today: true,
            };
        }
        state.tick_time();
        "Watered the crop. 💧".to_string()
    } else {
        "Nothing to water there.".to_string()
    }
}

pub fn harvest(state: &mut GameState, dir: Option<Direction>) -> String {
    let (x, y) = target_pos(state, dir);
    let map = state.get_map(state.player.location);
    if !in_bounds(map, x, y) {
        return "Nothing to harvest there.".to_string();
    }
    let tile = &map[y][x];
    match tile {
        TileType::Crop {
            crop_type,
            days_grown,
            ..
        } if *days_grown >= crop_type.maturity_days() => {
            let ct = *crop_type;
            state.get_map_mut(state.player.location)[y][x] = TileType::Soil;
            match ct {
                CropType::Carrot => state.inventory.carrots += 1,
                CropType::Strawberry => state.inventory.strawberries += 1,
                CropType::Cauliflower => state.inventory.cauliflowers += 1,
                CropType::Flower => state.inventory.flowers += 1,
            }
            state.tick_time();
            format!("Harvested a {}!", ct.emoji())
        }
        TileType::Mushroom => {
            state.get_map_mut(state.player.location)[y][x] = TileType::Grass;
            state.inventory.mushrooms += 1;
            state.tick_time();
            "Collected a mushroom! 🍄".to_string()
        }
        TileType::Crop { .. } => "Not ready to harvest yet.".to_string(),
        _ => "Nothing to harvest there.".to_string(),
    }
}

fn crop_type_name(ct: CropType) -> &'static str {
    match ct {
        CropType::Carrot => "carrot",
        CropType::Strawberry => "strawberry",
        CropType::Cauliflower => "cauliflower",
        CropType::Flower => "flower",
    }
}
