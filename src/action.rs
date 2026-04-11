use rand::Rng;

use crate::state::GameState;
use crate::types::*;

pub fn execute(state: &mut GameState, args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: tinydew do <action> [args]".to_string();
    }

    let action = args[0];
    let arg = args.get(1).copied();

    match action {
        "move" => do_move(state, arg),
        "clear" => do_directional(state, arg, do_clear),
        "plant" => do_directional(state, arg, do_plant),
        "water" => do_directional(state, arg, do_water),
        "harvest" => do_directional(state, arg, do_harvest),
        "fish" => do_fish(state, arg),
        "buy" => do_buy(state, arg),
        "sell" => do_sell(state, arg),
        "sleep" => do_sleep(state),
        _ => format!("Unknown action: {}", action),
    }
}

fn do_directional(
    state: &mut GameState,
    dir_arg: Option<&str>,
    handler: fn(&mut GameState, Direction) -> String,
) -> String {
    let dir = match dir_arg {
        Some(s) => match parse_direction(s) {
            Some(d) => d,
            None => return format!("Invalid direction: {}", s),
        },
        None => state.player.direction,
    };
    handler(state, dir)
}

// --- Movement ---

fn do_move(state: &mut GameState, dir_arg: Option<&str>) -> String {
    let dir = match dir_arg {
        Some(s) => match parse_direction(s) {
            Some(d) => d,
            None => return format!("Invalid direction: {}", s),
        },
        None => return "Usage: move <up|down|left|right>".to_string(),
    };

    state.player.direction = dir;

    let map = state.maps.get(&state.player.region);
    let height = map.len();
    let width = map[0].len();

    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, width, height) {
        Some(pos) => pos,
        None => return "Can't move there.".to_string(),
    };

    let tile = &map[ty][tx];

    if matches!(tile, TileType::Wonder) {
        return "That is so beautiful. Let's enjoy it together in the game.".to_string();
    }

    if let TileType::Plant { crop, days_grown, .. } = tile {
        if *days_grown >= crop.days_to_mature() {
            return "A mature crop is in the way. Try harvesting it first.".to_string();
        }
    }

    // Check for region transitions
    let transition = match (&state.player.region, tile) {
        (Region::Farm, TileType::PathEast) => Some((Region::EastPath, 1, 2, Direction::Right, "You walk east along the path.")),
        (Region::EastPath, TileType::PathFarm) => Some((Region::Farm, 6, 5, Direction::Left, "You return to the farm.")),
        (Region::EastPath, TileType::PathSquare) => Some((Region::Square, 4, 3, Direction::Up, "You enter the square.")),
        (Region::Square, TileType::PathSquare) => Some((Region::EastPath, 5, 1, Direction::Down, "You head back to the east path.")),
        (Region::EastPath, TileType::PathSouthRiver) => Some((Region::SouthRiver, 2, 1, Direction::Down, "You walk south to the river.")),
        (Region::SouthRiver, TileType::PathSouthRiverGate) => Some((Region::EastPath, 2, 2, Direction::Up, "You head back to the east path.")),
        _ => None,
    };

    if let Some((region, x, y, facing, msg)) = transition {
        state.player.region = region;
        state.player.x = x;
        state.player.y = y;
        state.player.direction = facing;
        advance_time(state, 5);
        return msg.to_string();
    }

    if !tile.is_walkable() {
        return "Can't move there.".to_string();
    }

    state.player.x = tx;
    state.player.y = ty;
    advance_time(state, 5);

    format!("Moved {}.", dir)
}

// --- Farming actions ---

fn do_clear(state: &mut GameState, dir: Direction) -> String {
    if state.player.region != Region::Farm {
        return "You can't clear here.".to_string();
    }

    let map = state.maps.get(&state.player.region);
    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, map[0].len(), map.len()) {
        Some(pos) => pos,
        None => return "Can't clear there.".to_string(),
    };

    let tile = &map[ty][tx];
    match tile {
        TileType::Grass => {
            state.maps.get_mut(&state.player.region)[ty][tx] = TileType::Soil;
            advance_time(state, 5);
            "Cleared the ground.".to_string()
        }
        TileType::Plant { crop, days_grown, .. } if *days_grown < crop.days_to_mature() => {
            state.maps.get_mut(&state.player.region)[ty][tx] = TileType::Soil;
            advance_time(state, 5);
            "Cleared the ground.".to_string()
        }
        TileType::Soil => "The ground is already cleared.".to_string(),
        TileType::Plant { crop, days_grown, .. } if *days_grown >= crop.days_to_mature() => {
            "Can't clear a mature crop. Try harvesting it first.".to_string()
        }
        _ => "Can't clear that.".to_string(),
    }
}

fn do_plant(state: &mut GameState, dir: Direction) -> String {
    if state.player.region != Region::Farm {
        return "You can't plant here.".to_string();
    }
    if state.inventory.seeds == 0 {
        return "You don't have any seeds.".to_string();
    }

    let map = state.maps.get(&state.player.region);
    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, map[0].len(), map.len()) {
        Some(pos) => pos,
        None => return "Can't plant there.".to_string(),
    };

    if map[ty][tx] != TileType::Soil {
        return "Can only plant on soil.".to_string();
    }

    let mut rng = rand::thread_rng();
    let crop = match rng.gen_range(0..3) {
        0 => CropType::Carrot,
        1 => CropType::Strawberry,
        _ => CropType::Cauliflower,
    };

    state.inventory.seeds -= 1;
    state.maps.get_mut(&state.player.region)[ty][tx] = TileType::Plant {
        crop,
        days_grown: 0,
        watered: false,
    };

    advance_time(state, 5);
    format!("Planted a seed. A {} is growing! \u{1f331}", crop.name())
}

fn do_water(state: &mut GameState, dir: Direction) -> String {
    let region = state.player.region;
    let map = state.maps.get(&region);
    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, map[0].len(), map.len()) {
        Some(pos) => pos,
        None => return "Nothing to water here.".to_string(),
    };

    if !matches!(map[ty][tx], TileType::Plant { .. }) {
        return "Nothing to water here.".to_string();
    }

    if let TileType::Plant { watered, .. } = &mut state.maps.get_mut(&region)[ty][tx] {
        *watered = true;
    }

    advance_time(state, 5);
    "Watered the crop. \u{1f4a7}".to_string()
}

fn do_harvest(state: &mut GameState, dir: Direction) -> String {
    let region = state.player.region;
    let map = state.maps.get(&region);
    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, map[0].len(), map.len()) {
        Some(pos) => pos,
        None => return "Nothing to harvest here.".to_string(),
    };

    let tile = map[ty][tx].clone();
    match &tile {
        TileType::Plant { crop, days_grown, .. } if *days_grown >= crop.days_to_mature() => {
            if region != Region::Farm {
                return "You can't harvest crops here.".to_string();
            }
            let crop = *crop;
            state.maps.get_mut(&region)[ty][tx] = TileType::Plant {
                crop,
                days_grown: 0,
                watered: false,
            };
            match crop {
                CropType::Carrot => state.inventory.carrots += 1,
                CropType::Strawberry => state.inventory.strawberries += 1,
                CropType::Cauliflower => state.inventory.cauliflowers += 1,
            }
            advance_time(state, 5);
            format!("Harvested a {}! +1 {}", crop.name(), crop.emoji())
        }
        TileType::Mushroom => {
            state.maps.get_mut(&region)[ty][tx] = TileType::Soil;
            state.inventory.mushrooms += 1;
            advance_time(state, 5);
            "Foraged a mushroom! +1 \u{1f344}".to_string()
        }
        TileType::Flower => {
            state.maps.get_mut(&region)[ty][tx] = TileType::Soil;
            state.inventory.flowers += 1;
            advance_time(state, 5);
            "Foraged a flower! +1 \u{1f33a}".to_string()
        }
        _ => "Nothing to harvest here.".to_string(),
    }
}

// --- Fishing ---

fn do_fish(state: &mut GameState, dir_arg: Option<&str>) -> String {
    let dir = match dir_arg {
        Some(s) => match parse_direction(s) {
            Some(d) => d,
            None => return format!("Invalid direction: {}", s),
        },
        None => {
            // Auto-target: find adjacent fishable tile
            let map = state.maps.get(&state.player.region);
            let dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            let mut found = None;
            for d in dirs {
                if let Some((tx, ty)) =
                    target_pos(state.player.x, state.player.y, d, map[0].len(), map.len())
                {
                    if map[ty][tx].is_fishable() {
                        found = Some(d);
                        break;
                    }
                }
            }
            match found {
                Some(d) => d,
                None => return "Can't fish here.".to_string(),
            }
        }
    };

    let region = state.player.region;
    let map = state.maps.get(&region);
    let (tx, ty) = match target_pos(state.player.x, state.player.y, dir, map[0].len(), map.len()) {
        Some(pos) => pos,
        None => return "Can't fish here.".to_string(),
    };

    if !map[ty][tx].is_fishable() {
        return "Can't fish here.".to_string();
    }

    let is_bubble = map[ty][tx] == TileType::RiverBubble;

    if is_bubble {
        state.maps.get_mut(&region)[ty][tx] = TileType::River;
    }

    advance_time(state, 60);

    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(0..100);

    let (common_thresh, rare_thresh) = if is_bubble { (70, 90) } else { (40, 50) };

    if roll < common_thresh {
        state.inventory.common_fish += 1;
        "You caught a fish! \u{1f41f}".to_string()
    } else if roll < rare_thresh {
        state.inventory.rare_fish += 1;
        "You caught a rare fish! \u{1f420}".to_string()
    } else {
        "No bite...".to_string()
    }
}

// --- Economy ---

fn do_buy(state: &mut GameState, item_arg: Option<&str>) -> String {
    let item = match item_arg {
        Some(s) => s,
        None => return "What would you like to buy?".to_string(),
    };

    match item {
        "seed" => {
            if state.money >= 10 {
                state.money -= 10;
                state.inventory.seeds += 1;
                "Bought a seed. -$10".to_string()
            } else {
                "Not enough money.".to_string()
            }
        }
        _ => format!("Can't buy '{}'.", item),
    }
}

fn do_sell(state: &mut GameState, item_arg: Option<&str>) -> String {
    let item = match item_arg {
        Some(s) => s,
        None => return "What would you like to sell?".to_string(),
    };

    match item {
        "\u{1f353}" => sell_item(&mut state.inventory.strawberries, &mut state.money, 20, "strawberry", "\u{1f353}"),
        "\u{1f955}" => sell_item(&mut state.inventory.carrots, &mut state.money, 15, "carrot", "\u{1f955}"),
        "\u{1f966}" => sell_item(&mut state.inventory.cauliflowers, &mut state.money, 25, "cauliflower", "\u{1f966}"),
        "\u{1f344}" => sell_item(&mut state.inventory.mushrooms, &mut state.money, 25, "mushroom", "\u{1f344}"),
        "\u{1f33a}" => sell_item(&mut state.inventory.flowers, &mut state.money, 25, "flower", "\u{1f33a}"),
        "\u{1f41f}" => sell_item(&mut state.inventory.common_fish, &mut state.money, 10, "fish", "\u{1f41f}"),
        "\u{1f420}" => sell_item(&mut state.inventory.rare_fish, &mut state.money, 30, "rare fish", "\u{1f420}"),
        _ => format!("Can't sell '{}'.", item),
    }
}

fn sell_item(count: &mut u32, money: &mut i32, price: i32, name: &str, emoji: &str) -> String {
    if *count > 0 {
        *count -= 1;
        *money += price;
        format!("Sold a {}. +${}", name, price)
    } else {
        format!("You don't have any {} to sell.", emoji)
    }
}

// --- Sleep & Day Transition ---

fn do_sleep(state: &mut GameState) -> String {
    if !state.day_start_done {
        // Day already incremented at midnight but day-start hasn't run yet
        run_day_start(state);
    } else if state.time_minutes < 1440 {
        // Still same calendar day — advance to next day
        state.day += 1;
        run_day_start(state);
    }
    // else: crossed midnight and day-start already ran — just reposition

    state.time_minutes = 360; // 06:00

    state.player.x = 3;
    state.player.y = 3;
    state.player.region = Region::Farm;
    state.player.direction = Direction::Down;

    "You slept through the night. Good morning!".to_string()
}

fn run_day_start(state: &mut GameState) {
    // 1. Weather roll (with festival override)
    state.weather = roll_weather(state.day);

    // 2. Crop growth check + watered reset
    grow_crops(state);

    // 3. River bubble reset + spawn new bubble
    reset_river_bubbles(state);

    // 4. Random spawns
    do_random_spawns(state);

    // 5. Soil reverts to grass
    revert_soil_to_grass(state);

    // 6. Festival checks
    check_festival(state);

    state.day_start_done = true;
}

fn advance_time(state: &mut GameState, minutes: u32) {
    state.time_minutes += minutes;

    // Midnight: increment day, reset clock, but don't run day-start yet
    while state.time_minutes >= 1440 {
        state.time_minutes -= 1440;
        state.day += 1;
        state.day_start_done = false;
    }

    // 06:00 (360 min): auto-trigger day-start processing
    if !state.day_start_done && state.time_minutes >= 360 {
        run_day_start(state);
    }
}

pub fn roll_weather(day: u32) -> Weather {
    if day == 1 {
        return Weather::Sunny;
    }
    if day == 28 {
        return Weather::Sunny;
    }
    let hash = day.wrapping_mul(2654435761);
    let roll = hash % 100;
    if roll < 50 {
        Weather::Sunny
    } else if roll < 80 {
        Weather::Cloudy
    } else {
        Weather::Rainy
    }
}

fn grow_crops(state: &mut GameState) {
    let is_rainy = state.weather == Weather::Rainy;
    for region in Region::ALL {
        let map = state.maps.get_mut(&region);
        for row in map.iter_mut() {
            for tile in row.iter_mut() {
                if let TileType::Plant {
                    watered,
                    days_grown,
                    ..
                } = tile
                {
                    if is_rainy {
                        *watered = true;
                    }
                    if *watered {
                        *days_grown = days_grown.saturating_add(1);
                    }
                    *watered = false;
                }
            }
        }
    }
}

fn reset_river_bubbles(state: &mut GameState) {
    let map = state.maps.get_mut(&Region::SouthRiver);
    for row in map.iter_mut() {
        for tile in row.iter_mut() {
            if *tile == TileType::RiverBubble {
                *tile = TileType::River;
            }
        }
    }
    // Spawn a new bubble deterministically
    spawn_river_bubble(state);
}

fn spawn_river_bubble(state: &mut GameState) {
    let map = state.maps.get_mut(&Region::SouthRiver);
    let width = map[0].len();
    let hash = state.day.wrapping_mul(7919);
    let x = (hash as usize) % width;
    let y = 2 + ((hash as usize / width) % 2);
    if map[y][x] == TileType::River {
        map[y][x] = TileType::RiverBubble;
    }
}

fn do_random_spawns(state: &mut GameState) {
    let day = state.day;

    for (i, region) in Region::ALL.iter().enumerate() {
        let region_idx = i as u32;

        // Check for existing flower
        let has_flower = {
            let map = state.maps.get(region);
            map.iter()
                .flatten()
                .any(|t| matches!(t, TileType::Flower))
        };

        if !has_flower {
            let candidates = {
                let map = state.maps.get(region);
                collect_grass_candidates(map, *region)
            };
            if !candidates.is_empty() {
                let hash = det_hash(day, region_idx, 0);
                let idx = (hash as usize) % candidates.len();
                let (x, y) = candidates[idx];
                state.maps.get_mut(region)[y][x] = TileType::Flower;
            }
        }

        // Check for existing mushroom (re-check map since flower may have been placed)
        let has_mushroom = {
            let map = state.maps.get(region);
            map.iter()
                .flatten()
                .any(|t| matches!(t, TileType::Mushroom))
        };

        if !has_mushroom {
            let candidates = {
                let map = state.maps.get(region);
                collect_grass_candidates(map, *region)
            };
            if !candidates.is_empty() {
                let hash = det_hash(day, region_idx, 1);
                let idx = (hash as usize) % candidates.len();
                let (x, y) = candidates[idx];
                state.maps.get_mut(region)[y][x] = TileType::Mushroom;
            }
        }
    }
}

fn collect_grass_candidates(map: &[Vec<TileType>], region: Region) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TileType::Grass && !is_protected_tile(region, x, y) {
                candidates.push((x, y));
            }
        }
    }
    candidates
}

fn is_protected_tile(region: Region, x: usize, y: usize) -> bool {
    match region {
        Region::Farm => {
            // Wake-up position (3,3)
            x == 3 && y == 3
        }
        _ => false,
    }
}

fn det_hash(day: u32, region_idx: u32, type_idx: u32) -> u32 {
    day.wrapping_mul(31)
        .wrapping_add(region_idx.wrapping_mul(97))
        .wrapping_add(type_idx.wrapping_mul(53))
        .wrapping_mul(2654435761)
}

fn revert_soil_to_grass(state: &mut GameState) {
    for region in Region::ALL {
        let map = state.maps.get_mut(&region);
        for row in map.iter_mut() {
            for tile in row.iter_mut() {
                if *tile == TileType::Soil {
                    *tile = TileType::Grass;
                }
            }
        }
    }
}

fn check_festival(state: &mut GameState) {
    if state.season == Season::Spring && state.day == 28 {
        let map = state.maps.get_mut(&Region::Square);
        map[2][2] = TileType::Wonder;
    } else {
        let map = state.maps.get_mut(&Region::Square);
        if map[2][2] == TileType::Wonder {
            map[2][2] = TileType::Grass;
        }
    }
}
