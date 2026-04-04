use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::economy::{FishType, Inventory, SEED_PRICE};
use crate::entity::{Direction, Player};
use crate::map::{
    Location, RegionMap, create_east_path, create_farm, create_south_river, create_square,
};
use crate::tile::{CropData, CropType, TileType};
use crate::weather::{Weather, roll_weather};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub day: u32,
    pub time_hour: u32,
    pub time_minute: u32,
    pub weather: Weather,
    pub player: Player,
    pub inventory: Inventory,
    pub money: i32,
    pub maps: HashMap<String, RegionMap>,
    pub message: String,
    pub season: String,
}

impl GameState {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        maps.insert("Farm".to_string(), create_farm());
        maps.insert("EastPath".to_string(), create_east_path());
        maps.insert("Square".to_string(), create_square());
        maps.insert("SouthRiver".to_string(), create_south_river());

        Self {
            day: 1,
            time_hour: 6,
            time_minute: 0,
            weather: Weather::Sunny,
            player: Player::new(),
            inventory: Inventory::new(),
            money: 500,
            maps,
            message: "Welcome to TinyDew! A new day begins.".to_string(),
            season: "Spring".to_string(),
        }
    }

    pub fn current_map(&self) -> &RegionMap {
        let key = self.player.location.to_string();
        self.maps.get(&key).unwrap()
    }

    pub fn current_map_mut(&mut self) -> &mut RegionMap {
        let key = self.player.location.to_string();
        self.maps.get_mut(&key).unwrap()
    }

    pub fn is_night(&self) -> bool {
        self.time_hour >= 20 || self.time_hour < 6
    }

    pub fn weather_icon(&self) -> &str {
        if self.is_night() {
            "🌙"
        } else {
            self.weather.emoji()
        }
    }

    pub fn time_string(&self) -> String {
        format!("{:02}:{:02}", self.time_hour, self.time_minute)
    }

    pub fn advance_time(&mut self, minutes: u32) {
        self.time_minute += minutes;
        while self.time_minute >= 60 {
            self.time_minute -= 60;
            self.time_hour += 1;
        }
        // Don't auto-wrap past 24; the game continues past midnight
        // and suggests sleeping
        if self.time_hour >= 24 {
            self.time_hour %= 24;
        }

        if self.is_night() && self.time_hour < 6 {
            self.message = "It's very late... You should sleep.".to_string();
        }
    }

    fn is_butterfly_festival(&self) -> bool {
        self.season == "Spring" && self.day == 28
    }

    pub fn greeting_message(&self) -> String {
        if self.is_butterfly_festival() {
            return "Today is Butterfly Festival, enjoy it!".to_string();
        }

        if self.is_night() {
            "The stars are beautiful tonight.".to_string()
        } else {
            match self.weather {
                Weather::Sunny => "What a beautiful sunny day!".to_string(),
                Weather::Cloudy => "Clouds drift across the sky.".to_string(),
                Weather::Rainy => "Rain falls gently on the farm.".to_string(),
            }
        }
    }

    // === MOVEMENT ===

    pub fn do_move(&mut self, dir: Direction) {
        let (tx, ty) = self.player.target_pos(dir);
        self.player.direction = dir;

        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "You can't go that way.".to_string();
            return;
        }

        let target_tile = map.get(tx as usize, ty as usize).unwrap();

        // Check for Wonder tile
        if matches!(target_tile, TileType::Wonder) {
            self.message =
                "That is so beautiful. Let's enjoy it together in the game.".to_string();
            return;
        }

        if !target_tile.is_walkable() {
            if matches!(target_tile, TileType::Crop(d) if d.is_mature()) {
                self.message = "A mature crop is blocking the way. Try harvesting it first!"
                    .to_string();
            } else {
                self.message = "You can't walk there.".to_string();
            }
            return;
        }

        // Check for region transitions
        if let Some((new_loc, new_x, new_y, new_dir, msg)) =
            self.check_transition(tx as usize, ty as usize)
        {
            self.player.x = new_x;
            self.player.y = new_y;
            self.player.location = new_loc;
            self.player.direction = new_dir;
            self.message = msg;
            self.advance_time(5);
            return;
        }

        self.player.x = tx as usize;
        self.player.y = ty as usize;
        self.advance_time(5);
        self.message = self.greeting_message();
    }

    fn check_transition(
        &self,
        x: usize,
        y: usize,
    ) -> Option<(Location, usize, usize, Direction, String)> {
        let tile = self.current_map().get(x, y)?;
        match (&self.player.location, tile) {
            (Location::Farm, TileType::PathEast) => Some((
                Location::EastPath,
                1,
                2,
                Direction::Right,
                "You arrive at the East Path.".to_string(),
            )),
            (Location::EastPath, TileType::PathFarm) => Some((
                Location::Farm,
                6,
                5,
                Direction::Left,
                "You return to the Farm.".to_string(),
            )),
            (Location::EastPath, TileType::PathSquare) => Some((
                Location::Square,
                4,
                3,
                Direction::Up,
                "You enter the Town Square.".to_string(),
            )),
            (Location::Square, TileType::PathSquare) => Some((
                Location::EastPath,
                5,
                1,
                Direction::Down,
                "You return to the East Path.".to_string(),
            )),
            (Location::EastPath, TileType::PathSouthRiver) => Some((
                Location::SouthRiver,
                2,
                1,
                Direction::Down,
                "You arrive at the South River.".to_string(),
            )),
            (Location::SouthRiver, TileType::PathSouthRiverGate) => Some((
                Location::EastPath,
                2,
                2,
                Direction::Up,
                "You return to the East Path.".to_string(),
            )),
            _ => None,
        }
    }

    // === FARMING ACTIONS ===

    pub fn do_clear(&mut self, dir: Direction) {
        if self.player.location == Location::Square {
            self.message = "You can't clear here in the Square.".to_string();
            return;
        }
        if self.player.location != Location::Farm {
            self.message = "You can only clear on the Farm.".to_string();
            return;
        }

        let (tx, ty) = self.player.target_pos(dir);
        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "Nothing to clear there.".to_string();
            return;
        }

        let tile = map.get(tx as usize, ty as usize).unwrap();
        if !tile.is_clearable() {
            self.message = "You can't clear that.".to_string();
            return;
        }

        let map = self.current_map_mut();
        *map.get_mut(tx as usize, ty as usize).unwrap() = TileType::Soil;
        self.advance_time(5);
        self.message = "You cleared the ground into soil.".to_string();
    }

    pub fn do_plant(&mut self, dir: Direction) {
        if self.player.location != Location::Farm {
            self.message = "You can only plant on the Farm.".to_string();
            return;
        }

        if self.inventory.seeds == 0 {
            self.message = "You don't have any seeds!".to_string();
            return;
        }

        let (tx, ty) = self.player.target_pos(dir);
        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "Can't plant there.".to_string();
            return;
        }

        let tile = map.get(tx as usize, ty as usize).unwrap();
        if !tile.is_plantable() {
            self.message = "You can only plant on tilled soil.".to_string();
            return;
        }

        // Random crop type
        let crop_type = {
            let mut rng = rand::thread_rng();
            match rng.gen_range(0..4) {
                0 => CropType::Carrot,
                1 => CropType::Strawberry,
                2 => CropType::Cauliflower,
                _ => CropType::Flower,
            }
        };

        self.inventory.seeds -= 1;
        let map = self.current_map_mut();
        *map.get_mut(tx as usize, ty as usize).unwrap() = TileType::Crop(CropData::new(crop_type));
        self.advance_time(5);
        self.message = "You planted a seed! 🌱".to_string();
    }

    pub fn do_water(&mut self, dir: Direction) {
        let (tx, ty) = self.player.target_pos(dir);
        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "Nothing to water there.".to_string();
            return;
        }

        let tile = map.get(tx as usize, ty as usize).unwrap();
        if !matches!(tile, TileType::Crop(_)) {
            self.message = "There's no crop to water there.".to_string();
            return;
        }

        let map = self.current_map_mut();
        if let Some(TileType::Crop(data)) = map.get_mut(tx as usize, ty as usize) {
            if data.is_mature() {
                self.message = "This crop is already mature!".to_string();
                return;
            }
            data.watered_today = true;
            self.advance_time(5);
            self.message = "You watered the crop. 💧".to_string();
        }
    }

    pub fn do_harvest(&mut self, dir: Direction) {
        let (tx, ty) = self.player.target_pos(dir);
        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "Nothing to harvest there.".to_string();
            return;
        }

        let tile = map.get(tx as usize, ty as usize).unwrap().clone();

        match tile {
            TileType::Crop(ref data) if data.is_mature() => {
                let crop_type = data.crop_type;
                self.inventory.add_produce(crop_type);
                let map = self.current_map_mut();
                *map.get_mut(tx as usize, ty as usize).unwrap() = TileType::Grass;
                self.advance_time(5);
                self.message =
                    format!("You harvested {} {}!", crop_type.emoji(), crop_type_name(crop_type));
            }
            TileType::Crop(_) => {
                self.message = "This crop isn't mature yet.".to_string();
            }
            TileType::Mushroom => {
                self.inventory.mushrooms += 1;
                let map = self.current_map_mut();
                *map.get_mut(tx as usize, ty as usize).unwrap() = TileType::Grass;
                self.advance_time(5);
                self.message = "You picked a mushroom! 🍄".to_string();
            }
            _ => {
                self.message = "Nothing to harvest there.".to_string();
            }
        }
    }

    // === ECONOMY ===

    pub fn do_buy(&mut self, item: &str) {
        match item {
            "seed" => {
                if self.money >= SEED_PRICE {
                    self.money -= SEED_PRICE;
                    self.inventory.seeds += 1;
                    self.advance_time(5);
                    self.message =
                        format!("Bought 1 seed for ${SEED_PRICE}. Seeds: {}", self.inventory.seeds);
                } else {
                    self.message =
                        format!("Not enough money! Need ${SEED_PRICE}, have ${}.", self.money);
                }
            }
            _ => {
                self.message = format!("Unknown item: {item}");
            }
        }
    }

    pub fn do_sell(&mut self, emoji: &str) {
        match self.inventory.try_sell(emoji) {
            Ok(price) => {
                self.money += price;
                self.advance_time(5);
                self.message = format!("Sold {emoji} for ${price}! Money: ${}", self.money);
            }
            Err(e) => {
                self.message = e;
            }
        }
    }

    // === FISHING ===

    pub fn do_fish(&mut self, dir: Direction) {
        if self.player.location != Location::SouthRiver {
            self.message = "You need to be at the South River to fish.".to_string();
            return;
        }

        let (tx, ty) = self.player.target_pos(dir);
        let map = self.current_map();
        if !map.in_bounds(tx, ty) {
            self.message = "Can't fish there.".to_string();
            return;
        }

        let tile = map.get(tx as usize, ty as usize).unwrap();
        if !tile.is_fishable() {
            self.message = "There's no water to fish in that direction.".to_string();
            return;
        }

        let is_bubble = matches!(tile, TileType::RiverBubble);

        let mut rng = rand::thread_rng();
        let roll: u32 = rng.gen_range(0..100);

        self.advance_time(5);

        if is_bubble {
            // Bubble = guaranteed catch, higher rare chance
            if roll < 40 {
                self.inventory.add_fish(FishType::Rare);
                self.message = "Amazing! You caught a rare fish! 🐠".to_string();
            } else {
                self.inventory.add_fish(FishType::Common);
                self.message = "You caught a fish! 🐟".to_string();
            }
            // Remove bubble
            let map = self.current_map_mut();
            *map.get_mut(tx as usize, ty as usize).unwrap() = TileType::River;
        } else {
            // Normal: 60% catch, 10% rare
            if roll < 10 {
                self.inventory.add_fish(FishType::Rare);
                self.message = "Wow! You caught a rare fish! 🐠".to_string();
            } else if roll < 60 {
                self.inventory.add_fish(FishType::Common);
                self.message = "You caught a fish! 🐟".to_string();
            } else {
                self.message = "No bite... The fish got away.".to_string();
            }
        }
    }

    // === SLEEP ===

    pub fn do_sleep(&mut self) {
        // Advance to next morning 06:00
        self.day += 1;
        self.time_hour = 6;
        self.time_minute = 0;

        // Day-start processing
        self.day_start_processing();

        // Wake up at Farm (3,3)
        self.player.x = 3;
        self.player.y = 3;
        self.player.location = Location::Farm;
        self.player.direction = Direction::Down;

        self.message = self.greeting_message();
    }

    fn day_start_processing(&mut self) {
        // Weather roll
        self.weather = roll_weather(self.day);

        // Crop growth
        self.process_crop_growth();

        // River bubble reset
        self.reset_river_bubbles();

        // Random spawns
        self.process_spawns();

        // Festival checks
        self.process_festival();
    }

    fn process_crop_growth(&mut self) {
        let is_rainy = self.weather == Weather::Rainy;

        for map in self.maps.values_mut() {
            for row in &mut map.tiles {
                for tile in row.iter_mut() {
                    if let TileType::Crop(data) = tile {
                        // Rainy weather auto-waters
                        if is_rainy {
                            data.watered_today = true;
                        }
                        // If watered, grow
                        if data.watered_today {
                            data.days_grown += 1;
                        }
                        // Reset watered state for new day
                        data.watered_today = false;
                    }
                }
            }
        }
    }

    fn reset_river_bubbles(&mut self) {
        if let Some(river_map) = self.maps.get_mut("SouthRiver") {
            let mut rng = rand::thread_rng();
            for row in &mut river_map.tiles {
                for tile in row.iter_mut() {
                    if matches!(tile, TileType::River | TileType::RiverBubble) {
                        // Reset all to River first
                        *tile = TileType::River;
                    }
                }
            }
            // Spawn 1-3 bubbles randomly on river tiles
            let bubble_count = rng.gen_range(1..=3);
            let mut river_positions: Vec<(usize, usize)> = Vec::new();
            for (y, row) in river_map.tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if matches!(tile, TileType::River) {
                        river_positions.push((x, y));
                    }
                }
            }
            for _ in 0..bubble_count.min(river_positions.len()) {
                if river_positions.is_empty() {
                    break;
                }
                let idx = rng.gen_range(0..river_positions.len());
                let (bx, by) = river_positions.remove(idx);
                river_map.tiles[by][bx] = TileType::RiverBubble;
            }
        }
    }

    fn process_spawns(&mut self) {
        let mut rng = rand::thread_rng();

        // Process spawns for each region
        for (name, map) in &mut self.maps {
            // Check if region already has a flower or mushroom
            let has_spawn = map.tiles.iter().any(|row| {
                row.iter().any(|t| {
                    matches!(t, TileType::Mushroom)
                        || matches!(t, TileType::Crop(d) if d.crop_type == CropType::Flower && d.is_mature())
                })
            });

            if has_spawn {
                continue;
            }

            // Find valid empty grass tiles (not protected)
            let mut valid_tiles: Vec<(usize, usize)> = Vec::new();
            for (y, row) in map.tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if matches!(tile, TileType::Grass) {
                        // Skip protected positions
                        if is_protected_tile(name, x, y) {
                            continue;
                        }
                        valid_tiles.push((x, y));
                    }
                }
            }

            if valid_tiles.is_empty() {
                continue;
            }

            // 50% chance of spawn
            if rng.gen_range(0..100) < 50 {
                let idx = rng.gen_range(0..valid_tiles.len());
                let (sx, sy) = valid_tiles[idx];

                // Mushroom or flower (50/50)
                if rng.gen_bool(0.5) {
                    map.tiles[sy][sx] = TileType::Mushroom;
                } else {
                    // Spawn a mature flower
                    map.tiles[sy][sx] = TileType::Crop(CropData {
                        crop_type: CropType::Flower,
                        days_grown: CropType::Flower.days_to_mature(),
                        watered_today: false,
                    });
                }
            }
        }
    }

    fn process_festival(&mut self) {
        if self.is_butterfly_festival() {
            // Place Wonder tile at Square (2,2)
            if let Some(square_map) = self.maps.get_mut("Square") {
                square_map.tiles[2][2] = TileType::Wonder;
            }
        } else {
            // Reset Wonder back to Grass at Square (2,2) if it exists
            if let Some(square_map) = self.maps.get_mut("Square") {
                if matches!(square_map.tiles[2][2], TileType::Wonder) {
                    square_map.tiles[2][2] = TileType::Grass;
                }
            }
        }
    }
}

fn is_protected_tile(region: &str, x: usize, y: usize) -> bool {
    match region {
        "Farm" => {
            // Protect house (2,2) and wake-up position (3,3)
            (x == 2 && y == 2) || (x == 3 && y == 3)
        }
        _ => false,
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
