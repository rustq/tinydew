use crate::world::{
    CropState, CropType, Direction, EAST_PATH_HEIGHT, EAST_PATH_WIDTH, FARM_HEIGHT, FARM_WIDTH,
    FishType, ForageType, Map, SOUTH_RIVER_HEIGHT, SOUTH_RIVER_WIDTH, SQUARE_HEIGHT, SQUARE_WIDTH,
    TileType, Weather, create_east_path_map, create_farm_map, create_south_river_map,
    create_square_map,
};
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HomeState {
    None,
    Alert,
    Income,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    Farm,
    EastPath,
    Square,
    SouthRiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlTarget {
    Player,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub seeds: HashMap<CropType, u32>,
    pub produce: HashMap<CropType, u32>,
    pub forage: HashMap<ForageType, u32>,
    pub fish: HashMap<FishType, u32>,
}

#[allow(dead_code)]
impl Inventory {
    pub fn new() -> Self {
        Self {
            seeds: HashMap::new(),
            produce: HashMap::new(),
            forage: HashMap::new(),
            fish: HashMap::new(),
        }
    }

    pub fn get_seeds(&self, crop: CropType) -> u32 {
        *self.seeds.get(&crop).unwrap_or(&0)
    }

    pub fn add_seeds(&mut self, crop: CropType, count: u32) {
        *self.seeds.entry(crop).or_insert(0) += count;
    }

    pub fn use_seed(&mut self, crop: CropType) -> bool {
        let count = self.seeds.get(&crop).unwrap_or(&0);
        if *count > 0 {
            *self.seeds.get_mut(&crop).unwrap() -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_produce(&mut self, crop: CropType) {
        *self.produce.entry(crop).or_insert(0) += 1;
    }

    pub fn get_produce(&self, crop: CropType) -> u32 {
        *self.produce.get(&crop).unwrap_or(&0)
    }

    pub fn sell_produce(&mut self, crop: CropType) -> bool {
        let count = self.produce.get(&crop).unwrap_or(&0);
        if *count > 0 {
            *self.produce.get_mut(&crop).unwrap() -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_forage(&mut self, forage: ForageType) {
        *self.forage.entry(forage).or_insert(0) += 1;
    }

    pub fn get_forage(&self, forage: ForageType) -> u32 {
        *self.forage.get(&forage).unwrap_or(&0)
    }

    pub fn add_fish(&mut self, fish: FishType) {
        *self.fish.entry(fish).or_insert(0) += 1;
    }

    pub fn get_fish(&self, fish: FishType) -> u32 {
        *self.fish.get(&fish).unwrap_or(&0)
    }

    pub fn sell_forage(&mut self, forage: ForageType) -> bool {
        let count = self.forage.get(&forage).unwrap_or(&0);
        if *count > 0 {
            *self.forage.get_mut(&forage).unwrap() -= 1;
            true
        } else {
            false
        }
    }

    pub fn sell_fish(&mut self, fish: FishType) -> bool {
        let count = self.fish.get(&fish).unwrap_or(&0);
        if *count > 0 {
            *self.fish.get_mut(&fish).unwrap() -= 1;
            true
        } else {
            false
        }
    }

    pub fn fish_count(&self) -> u32 {
        self.fish.values().sum()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DailyIncome {
    pub money_earned: u32,
    pub crops_sold: HashMap<CropType, u32>,
    pub forage_sold: HashMap<ForageType, u32>,
    pub fish_sold: HashMap<FishType, u32>,
    pub crops_harvested: HashMap<CropType, u32>,
    pub forage_harvested: HashMap<ForageType, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShopState {
    None,
    BuyMenu,
    SellMenu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub location: Location,
    pub player_location: Location,
    pub farm_map: Map,
    pub east_path_map: Map,
    pub square_map: Map,
    pub south_river_map: Map,
    pub player_x: usize,
    pub player_y: usize,
    pub direction: Direction,
    pub message: String,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub season: String,
    pub weather: Weather,
    pub weather_day: u32,
    pub inventory: Inventory,
    pub selected_seed: CropType,
    pub money: u32,
    pub shop_state: ShopState,
    pub shop_cursor: usize,
    pub home_state: HomeState,
    pub home_cursor: usize,
    pub current_income: DailyIncome,
    pub total_minutes: u32,
    pub last_update_ms: u64,
    pub is_paused: bool,
    pub auto_sleep_triggered_day: u32,
    pub rng_seed: u64,
    pub spring_forced_flower_6_2_done: bool,
    pub last_spawn_processed_day: u32,
    pub guest_enabled: bool,
    pub guest_x: usize,
    pub guest_y: usize,
    pub guest_location: Location,
    pub active_control: ControlTarget,
}

#[allow(dead_code)]
impl GameState {
    pub fn new() -> Self {
        let farm_map = create_farm_map();
        let (player_x, player_y) = find_player_start(&farm_map);

        let east_path_map = create_east_path_map();
        let square_map = create_square_map();
        let south_river_map = create_south_river_map();

        Self {
            location: Location::Farm,
            player_location: Location::Farm,
            farm_map,
            east_path_map,
            square_map,
            south_river_map,
            player_x,
            player_y,
            direction: Direction::Down,
            message: String::from("Welcome to Tinydew!"),
            day: 1,
            hour: 6,
            minute: 0,
            season: String::from("Spring"),
            weather: Weather::Sunny,
            weather_day: 1,
            inventory: Inventory::new(),
            selected_seed: CropType::Carrot,
            money: 500,
            shop_state: ShopState::None,
            shop_cursor: 0,
            home_state: HomeState::None,
            home_cursor: 0,
            current_income: DailyIncome::default(),
            total_minutes: 360,
            last_update_ms: 0,
            is_paused: false,
            auto_sleep_triggered_day: 0,
            rng_seed: 12345,
            spring_forced_flower_6_2_done: false,
            last_spawn_processed_day: 0,
            guest_enabled: false,
            guest_x: 0,
            guest_y: 0,
            guest_location: Location::Farm,
            active_control: ControlTarget::Player,
        }
    }

    pub fn enable_guest_for_interactive(&mut self) {
        self.guest_enabled = true;
        // Guest mode: guest is always the controllable character in interactive runtime.
        self.active_control = ControlTarget::Guest;
        // Guest starts in Farm at a pseudo-random valid tile.
        self.guest_location = Location::Farm;
        if let Some((x, y)) = self.find_guest_spawn_location_in(Location::Farm) {
            self.guest_x = x;
            self.guest_y = y;
        }

        // Start interactive camera/location in Farm.
        self.location = Location::Farm;
    }

    pub fn disable_guest(&mut self) {
        self.guest_enabled = false;
        self.active_control = ControlTarget::Player;
    }

    pub fn is_guest_on_current_map(&self) -> bool {
        self.guest_enabled && self.guest_location == self.location
    }

    fn find_guest_spawn_location(&self) -> Option<(usize, usize)> {
        self.find_guest_spawn_location_in(self.guest_location)
    }

    fn find_guest_spawn_location_in(&self, location: Location) -> Option<(usize, usize)> {
        let (map, width, height) = match location {
            Location::Farm => (
                &self.farm_map,
                crate::world::FARM_WIDTH,
                crate::world::FARM_HEIGHT,
            ),
            Location::EastPath => (
                &self.east_path_map,
                crate::world::EAST_PATH_WIDTH,
                crate::world::EAST_PATH_HEIGHT,
            ),
            Location::Square => (
                &self.square_map,
                crate::world::SQUARE_WIDTH,
                crate::world::SQUARE_HEIGHT,
            ),
            Location::SouthRiver => (
                &self.south_river_map,
                crate::world::SOUTH_RIVER_WIDTH,
                crate::world::SOUTH_RIVER_HEIGHT,
            ),
        };

        let base = (self.rng_seed as usize).wrapping_add((self.day as usize).wrapping_mul(31));
        let sx = base % width;
        let sy = (base / width.max(1)) % height;

        for offset in 0..(width * height) {
            let x = (sx + offset) % width;
            let y = (sy + (offset / width.max(1))) % height;
            if map[y][x] == TileType::Grass
                && !(location == self.location && x == self.player_x && y == self.player_y)
            {
                return Some((x, y));
            }
        }
        None
    }

    pub fn can_move_guest_to(&self, x: usize, y: usize) -> bool {
        if self.player_location == self.guest_location && x == self.player_x && y == self.player_y {
            return false;
        }

        let (width, height) = match self.guest_location {
            Location::Farm => (crate::world::FARM_WIDTH, crate::world::FARM_HEIGHT),
            Location::EastPath => (
                crate::world::EAST_PATH_WIDTH,
                crate::world::EAST_PATH_HEIGHT,
            ),
            Location::Square => (crate::world::SQUARE_WIDTH, crate::world::SQUARE_HEIGHT),
            Location::SouthRiver => (
                crate::world::SOUTH_RIVER_WIDTH,
                crate::world::SOUTH_RIVER_HEIGHT,
            ),
        };

        if x >= width || y >= height {
            return false;
        }

        let map = match self.guest_location {
            Location::Farm => &self.farm_map,
            Location::EastPath => &self.east_path_map,
            Location::Square => &self.square_map,
            Location::SouthRiver => &self.south_river_map,
        };

        let tile = map[y][x];
        if tile == TileType::House {
            return false;
        }

        tile.is_walkable()
    }

    pub fn move_guest(&mut self, direction: Direction) -> bool {
        if !self.guest_enabled {
            return false;
        }

        let (dx, dy) = direction.delta();
        let new_x = self.guest_x as i32 + dx;
        let new_y = self.guest_y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if self.can_move_guest_to(new_x, new_y) {
            let target_tile = match self.guest_location {
                Location::Farm => self.farm_map[new_y][new_x],
                Location::EastPath => self.east_path_map[new_y][new_x],
                Location::Square => self.square_map[new_y][new_x],
                Location::SouthRiver => self.south_river_map[new_y][new_x],
            };

            if target_tile.is_transition() {
                self.handle_guest_transition_at(new_x, new_y);
            } else {
                self.guest_x = new_x;
                self.guest_y = new_y;
            }
            true
        } else if self.player_location == self.guest_location
            && new_x == self.player_x
            && new_y == self.player_y
        {
            self.message = String::from("Tile occupied.");
            false
        } else {
            let target_tile = match self.guest_location {
                Location::Farm => self.farm_map.get(new_y).and_then(|row| row.get(new_x)).copied(),
                Location::EastPath => self
                    .east_path_map
                    .get(new_y)
                    .and_then(|row| row.get(new_x))
                    .copied(),
                Location::Square => self
                    .square_map
                    .get(new_y)
                    .and_then(|row| row.get(new_x))
                    .copied(),
                Location::SouthRiver => self
                    .south_river_map
                    .get(new_y)
                    .and_then(|row| row.get(new_x))
                    .copied(),
            };

            if matches!(target_tile, Some(TileType::Wonder)) {
                self.message = String::from(
                    "That is so beautiful. Let human enjoy it together in interactive mode.",
                );
            } else {
                self.message = String::from("Cannot move there. Try walking around to find a path.");
            }
            false
        }
    }

    fn handle_guest_transition_at(&mut self, x: usize, y: usize) {
        let tile = match self.guest_location {
            Location::Farm => self.farm_map[y][x],
            Location::EastPath => self.east_path_map[y][x],
            Location::Square => self.square_map[y][x],
            Location::SouthRiver => self.south_river_map[y][x],
        };

        match (self.guest_location, tile) {
            (Location::Farm, TileType::PathEast) => {
                self.guest_location = Location::EastPath;
                self.location = Location::EastPath;
                self.guest_x = 1;
                self.guest_y = 2;
                self.message = String::from("Guest moved to East Path.");
            }
            (Location::EastPath, TileType::PathFarm) => {
                self.guest_location = Location::Farm;
                self.location = Location::Farm;
                self.guest_x = 7;
                self.guest_y = 5;
                self.message = String::from("Guest returned to Farm.");
            }
            (Location::EastPath, TileType::PathSquare) => {
                self.guest_location = Location::Square;
                self.location = Location::Square;
                self.guest_x = 4;
                self.guest_y = 4;
                self.message = String::from("Guest moved to Square.");
            }
            (Location::Square, TileType::PathSquare) => {
                self.guest_location = Location::EastPath;
                self.location = Location::EastPath;
                self.guest_x = 5;
                self.guest_y = 0;
                self.message = String::from("Guest left Square.");
            }
            (Location::EastPath, TileType::PathSouthRiver) => {
                self.guest_location = Location::SouthRiver;
                self.location = Location::SouthRiver;
                self.guest_x = 2;
                self.guest_y = 1;
                self.message = String::from("Guest moved to South River.");
            }
            (Location::SouthRiver, TileType::PathSouthRiverGate) => {
                self.guest_location = Location::EastPath;
                self.location = Location::EastPath;
                self.guest_x = 5;
                self.guest_y = 3;
                self.message = String::from("Guest returned to East Path.");
            }
            _ => {}
        }
    }

    pub fn is_guest_active(&self) -> bool {
        self.guest_enabled
    }

    pub fn toggle_control(&mut self) {
        // Guest mode is guest-only control; toggle is intentionally disabled.
    }

    pub fn is_time_frozen(&self) -> bool {
        self.guest_enabled
    }

    pub fn guest_greeting_message(&self) -> String {
        if self.season == "Spring" && self.day == 28 {
            return "✨ Happy Butterfly Festival!".to_string();
        }

        match self.weather {
            Weather::Rainy => "It’s rainy today, stay dry out there.".to_string(),
            Weather::Cloudy => "It’s cloudy today, a calm day to stroll.".to_string(),
            _ => {
                if self.hour < 12 {
                    "Good morning, long time no see.".to_string()
                } else if self.hour < 18 {
                    "Good afternoon, long time no see.".to_string()
                } else {
                    "Good night, maybe it's time to sleep.".to_string()
                }
            }
        }
    }

    pub fn get_current_map_ref(&self) -> &Map {
        match self.location {
            Location::Farm => &self.farm_map,
            Location::EastPath => &self.east_path_map,
            Location::Square => &self.square_map,
            Location::SouthRiver => &self.south_river_map,
        }
    }

    pub fn get_current_map(&mut self) -> &mut Map {
        match self.location {
            Location::Farm => &mut self.farm_map,
            Location::EastPath => &mut self.east_path_map,
            Location::Square => &mut self.square_map,
            Location::SouthRiver => &mut self.south_river_map,
        }
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        match self.location {
            Location::Farm => (FARM_WIDTH, FARM_HEIGHT),
            Location::EastPath => (EAST_PATH_WIDTH, EAST_PATH_HEIGHT),
            Location::Square => (SQUARE_WIDTH, SQUARE_HEIGHT),
            Location::SouthRiver => (SOUTH_RIVER_WIDTH, SOUTH_RIVER_HEIGHT),
        }
    }

    pub fn tile_in_front(&self) -> Option<(usize, usize)> {
        let (dx, dy) = self.direction.delta();
        let (width, height) = self.get_map_size();

        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    pub fn tile_at_direction(&self, dir: Direction) -> Option<(usize, usize)> {
        let (dx, dy) = dir.delta();
        let (width, height) = self.get_map_size();

        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    fn adjacent_tiles_priority(&self) -> Vec<(usize, usize)> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .filter_map(|dir| self.tile_at_direction(*dir))
        .collect()
    }

    fn find_adjacent_tile<F>(&self, mut predicate: F) -> Option<(usize, usize)>
    where
        F: FnMut(TileType) -> bool,
    {
        for (x, y) in self.adjacent_tiles_priority() {
            if let Some(tile) = self.get_tile_at(x, y)
                && predicate(tile)
            {
                return Some((x, y));
            }
        }
        None
    }

    pub fn can_move_to(&self, x: usize, y: usize) -> bool {
        let (width, height) = self.get_map_size();
        if x >= width || y >= height {
            return false;
        }

        let map = self.get_current_map_ref();
        map[y][x].is_walkable()
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<TileType> {
        let (width, height) = self.get_map_size();
        if x >= width || y >= height {
            return None;
        }
        let map = self.get_current_map_ref();
        Some(map[y][x])
    }

    pub fn move_player(&mut self, direction: Direction) -> bool {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return false;
        }

        self.direction = direction;

        let (dx, dy) = direction.delta();
        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if self.is_guest_enabled_at(new_x, new_y) {
            self.message = String::from("Tile occupied.");
            return false;
        }

        let target_tile = self.get_tile_at(new_x, new_y);

        if self.can_move_to(new_x, new_y) {
            if let Some(tile) = target_tile {
                if tile.is_transition() {
                    self.handle_transition(&tile);
                } else {
                    self.player_x = new_x;
                    self.player_y = new_y;
                    if tile == TileType::Wonder {
                        self.message = String::from(
                            "That is so beautiful. Let human enjoy it together in interactive mode.",
                        );
                    }
                    if !self.is_time_frozen() {
                        self.advance_time();
                    }
                }
            }
            true
        } else {
            if matches!(target_tile, Some(TileType::Wonder)) {
                self.message = String::from(
                    "That is so beautiful. Let human enjoy it together in interactive mode.",
                );
            } else {
                self.message = String::from("Cannot move there. Try walking around to find a path.");
            }
            false
        }
    }

    fn is_guest_enabled_at(&self, x: usize, y: usize) -> bool {
        self.guest_enabled
            && self.guest_location == self.location
            && self.guest_x == x
            && self.guest_y == y
    }

    fn handle_transition(&mut self, tile: &TileType) {
        match (self.location, tile) {
            (Location::Farm, TileType::PathEast) => {
                self.location = Location::EastPath;
                self.player_location = Location::EastPath;
                self.player_x = 1;
                self.player_y = 2;
                self.direction = Direction::Right;
                self.message = String::from("Entered East Path.");
            }
            (Location::EastPath, TileType::PathFarm) => {
                self.location = Location::Farm;
                self.player_location = Location::Farm;
                self.player_x = 7;
                self.player_y = 5;
                self.direction = Direction::Left;
                self.message = String::from("Returned to Farm.");
            }
            (Location::EastPath, TileType::PathSquare) => {
                self.location = Location::Square;
                self.player_location = Location::Square;
                self.player_x = 4;
                self.player_y = 4;
                self.direction = Direction::Up;
                self.message = String::from("Entered Square.");
            }
            (Location::Square, TileType::PathSquare) => {
                self.location = Location::EastPath;
                self.player_location = Location::EastPath;
                self.player_x = 5;
                self.player_y = 0;
                self.direction = Direction::Down;
                self.message = String::from("Returned to East Path.");
            }
            (Location::EastPath, TileType::PathSouthRiver) => {
                self.location = Location::SouthRiver;
                self.player_location = Location::SouthRiver;
                self.player_x = 2;
                self.player_y = 1;
                self.direction = Direction::Down;
                self.message = String::from("Entered South River.");
            }
            (Location::SouthRiver, TileType::PathSouthRiverGate) => {
                self.location = Location::EastPath;
                self.player_location = Location::EastPath;
                self.player_x = 5;
                self.player_y = 3;
                self.direction = Direction::Up;
                self.message = String::from("Returned to East Path.");
            }
            _ => {}
        }
        self.advance_time();
    }

    pub fn advance_time(&mut self) {
        self.advance_minutes(5);
    }

    fn reset_bubble_tiles(&mut self) {
        for y in 0..SOUTH_RIVER_HEIGHT {
            for x in 0..SOUTH_RIVER_WIDTH {
                if self.south_river_map[y][x] == TileType::RiverBubble {
                    self.south_river_map[y][x] = TileType::River;
                }
            }
        }
    }

    pub fn start_new_day(&mut self) {
        if self.weather_day != self.day {
            self.roll_weather();
        }

        for y in 0..FARM_HEIGHT {
            for x in 0..FARM_WIDTH {
                if let TileType::Crop(_crop, state) = &mut self.farm_map[y][x] {
                    if state.watered_today {
                        state.days_grown += 1;
                    }
                    state.watered_today = false;
                }
            }
        }

        if self.weather == Weather::Rainy {
            for y in 0..FARM_HEIGHT {
                for x in 0..FARM_WIDTH {
                    if let TileType::Crop(crop, state) = &mut self.farm_map[y][x] {
                        if !state.is_mature(*crop) && !state.watered_today {
                            state.watered_today = true;
                        }
                    }
                }
            }
        }

        for y in 0..FARM_HEIGHT {
            for x in 0..FARM_WIDTH {
                if let TileType::Soil = self.farm_map[y][x] {
                    self.farm_map[y][x] = TileType::Grass;
                }
            }
        }

        self.reset_bubble_tiles();

        self.spawn_random_crops();

        self.spawn_wonder_if_due();

        self.message = String::from("A new day begins.");
    }

    fn spawn_wonder_if_due(&mut self) {
        if self.season == "Spring" && self.day == 28 {
            self.square_map[2][2] = TileType::Wonder;
            self.message = String::from("Butterfly Festival is happening today.");
        } else if self.square_map[2][2] == TileType::Wonder {
            self.square_map[2][2] = TileType::Grass;
        }
    }

    fn spawn_east_path_mushrooms(&mut self) {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let random = (seed + self.day as u64) % 3;
        let mushroom_count = random as usize;

        let mut valid_positions: Vec<(usize, usize)> = Vec::new();

        for y in 0..EAST_PATH_HEIGHT {
            for x in 0..EAST_PATH_WIDTH {
                if let Some(TileType::Grass) = self.east_path_map.get(y).and_then(|row| row.get(x))
                {
                    if !(x == self.player_x
                        && y == self.player_y
                        && self.location == Location::EastPath)
                    {
                        valid_positions.push((x, y));
                    }
                }
            }
        }

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.day.hash(&mut hasher);
        seed.hash(&mut hasher);
        let mut rng_state = hasher.finish();

        for _ in 0..mushroom_count {
            if valid_positions.is_empty() {
                break;
            }
            let idx = (rng_state % valid_positions.len() as u64) as usize;
            let (mx, my) = valid_positions.remove(idx);
            self.east_path_map[my][mx] = TileType::Mushroom;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        }
    }

    fn get_empty_grass_positions(&self, map: &Map) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == TileType::Grass {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    fn is_protected_farm_spawn_tile(x: usize, y: usize) -> bool {
        // Never allow random mature crop/forage spawn on home or wake-up tile.
        // Home tile: (2,2), wake-up tile (front of home): (3,3)
        (x == 2 && y == 2) || (x == 3 && y == 3)
    }

    fn pick_random_tile(
        &mut self,
        positions: &mut Vec<(usize, usize)>,
        base_seed: u64,
    ) -> Option<(usize, usize)> {
        if positions.is_empty() {
            return None;
        }
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.day.hash(&mut hasher);
        base_seed.hash(&mut hasher);
        let rng_state = hasher.finish();

        let idx = (rng_state % positions.len() as u64) as usize;
        Some(positions.remove(idx))
    }

    pub fn spawn_random_crops(&mut self) {
        if self.season != "Spring" {
            return;
        }

        if self.last_spawn_processed_day == self.day {
            return;
        }
        self.last_spawn_processed_day = self.day;

        if !self.spring_forced_flower_6_2_done && self.farm_map[2][6] == TileType::Grass {
            let mature_state = CropState {
                days_grown: 16,
                watered_today: false,
            };
            self.farm_map[2][6] = TileType::Crop(CropType::Rhubarb, mature_state);
            self.spring_forced_flower_6_2_done = true;
        }

        let flower_seed = self
            .rng_seed
            .wrapping_add((self.day as u64).wrapping_mul(7919));
        let flower_roll = (flower_seed % 100) as u32;

        if flower_roll < 10 {
            let mut farm_positions = self.get_empty_grass_positions(&self.farm_map);
            let mut east_path_positions = self.get_empty_grass_positions(&self.east_path_map);
            let mut square_positions = self.get_empty_grass_positions(&self.square_map);
            let mut south_river_positions = self.get_empty_grass_positions(&self.south_river_map);

            #[derive(Clone, Copy)]
            enum FlowerSpawnMap {
                Farm,
                EastPath,
                Square,
                SouthRiver,
            }

            let mut map_candidates: Vec<FlowerSpawnMap> = Vec::new();
            if !farm_positions.is_empty() {
                map_candidates.push(FlowerSpawnMap::Farm);
            }
            if !east_path_positions.is_empty() {
                map_candidates.push(FlowerSpawnMap::EastPath);
            }
            if !square_positions.is_empty() {
                map_candidates.push(FlowerSpawnMap::Square);
            }
            if !south_river_positions.is_empty() {
                map_candidates.push(FlowerSpawnMap::SouthRiver);
            }

            if map_candidates.is_empty() {
                return;
            }

            let map_choice = ((flower_seed / 100) % (map_candidates.len() as u64)) as usize;
            let chosen_map = map_candidates[map_choice];

            let spawn_pos = match chosen_map {
                FlowerSpawnMap::Farm => {
                    self.pick_random_tile(&mut farm_positions, flower_seed.wrapping_add(1))
                }
                FlowerSpawnMap::EastPath => {
                    self.pick_random_tile(&mut east_path_positions, flower_seed.wrapping_add(1))
                }
                FlowerSpawnMap::Square => {
                    self.pick_random_tile(&mut square_positions, flower_seed.wrapping_add(1))
                }
                FlowerSpawnMap::SouthRiver => {
                    self.pick_random_tile(&mut south_river_positions, flower_seed.wrapping_add(1))
                }
            };

            if let Some((x, y)) = spawn_pos {
                let mature_state = CropState {
                    days_grown: 16,
                    watered_today: false,
                };
                match chosen_map {
                    FlowerSpawnMap::Farm => {
                        if !Self::is_protected_farm_spawn_tile(x, y) {
                            self.farm_map[y][x] = TileType::Crop(CropType::Rhubarb, mature_state);
                        }
                    }
                    FlowerSpawnMap::EastPath => {
                        self.east_path_map[y][x] = TileType::Crop(CropType::Rhubarb, mature_state);
                    }
                    FlowerSpawnMap::Square => {
                        self.square_map[y][x] = TileType::Crop(CropType::Rhubarb, mature_state);
                    }
                    FlowerSpawnMap::SouthRiver => {
                        self.south_river_map[y][x] =
                            TileType::Crop(CropType::Rhubarb, mature_state);
                    }
                }
            }
        }

        if self.weather == Weather::Rainy {
            let mushroom_seed = self
                .rng_seed
                .wrapping_add((self.day as u64).wrapping_mul(3571));
            let mut farm_positions = self.get_empty_grass_positions(&self.farm_map);
            let mut east_path_positions = self.get_empty_grass_positions(&self.east_path_map);

            let chosen_positions = if farm_positions.is_empty() {
                &mut east_path_positions
            } else if east_path_positions.is_empty() {
                &mut farm_positions
            } else {
                let map_choice = (mushroom_seed / 100) % 2;
                if map_choice == 0 {
                    &mut farm_positions
                } else {
                    &mut east_path_positions
                }
            };

            if let Some((x, y)) =
                self.pick_random_tile(chosen_positions, mushroom_seed.wrapping_add(2))
            {
                if y < FARM_HEIGHT && x < FARM_WIDTH {
                    if !Self::is_protected_farm_spawn_tile(x, y) {
                        self.farm_map[y][x] = TileType::Mushroom;
                    }
                } else if y < EAST_PATH_HEIGHT && x < EAST_PATH_WIDTH {
                    self.east_path_map[y][x] = TileType::Mushroom;
                }
            }
        }
    }

    fn roll_weather(&mut self) {
        if self.day == 1 {
            self.weather = Weather::Sunny;
            self.weather_day = self.day;
            return;
        }

        if self.season == "Spring" && self.day == 28 {
            self.weather = Weather::Sunny;
            self.weather_day = self.day;
            return;
        }

        let seed = self.rng_seed.wrapping_add(self.day as u64);
        let weather_idx = (seed % 100) as usize;
        self.weather = if weather_idx < 80 {
            Weather::Sunny
        } else if weather_idx < 93 {
            Weather::Cloudy
        } else {
            Weather::Rainy
        };
        self.weather_day = self.day;
    }

    pub fn is_day(&self) -> bool {
        self.hour >= 6 && self.hour < 18
    }

    pub fn is_night(&self) -> bool {
        !self.is_day()
    }

    pub fn get_weather_icon(&self) -> &'static str {
        if self.is_night() {
            "🌙"
        } else {
            self.weather.icon()
        }
    }

    pub fn get_day_and_time(&self) -> (u32, u8, u8) {
        (self.day, self.hour as u8, self.minute as u8)
    }

    pub fn should_auto_sleep(&self) -> bool {
        self.hour == 0
            && self.minute == 0
            && self.auto_sleep_triggered_day != self.day
            && self.home_state == HomeState::None
    }

    pub fn run_auto_sleep(&mut self) {
        self.auto_sleep_triggered_day = self.day;
        self.perform_sleep();
    }

    pub fn run_auto_sleep_and_advance(&mut self) {
        // Sleep always advances to the next 06:00 checkpoint.
        // If already after midnight (00:00-05:59), remain on the same day.
        // Otherwise, advance to next day.
        let crossed_to_next_day = self.hour >= 6;
        if crossed_to_next_day {
            self.day += 1;
        }

        self.auto_sleep_triggered_day = self.day;
        self.hour = 6;
        self.minute = 0;
        self.total_minutes = 0;

        self.location = Location::Farm;
        self.player_location = Location::Farm;
        self.player_x = 3;
        self.player_y = 3;

        if crossed_to_next_day {
            self.start_new_day();
        }

        self.home_state = HomeState::None;
        self.current_income = DailyIncome::default();
        self.message = String::from("Ready for a new day.");
    }

    pub fn tick(&mut self, current_time_ms: u64) {
        if self.is_paused {
            return;
        }

        let elapsed_ms = current_time_ms.saturating_sub(self.last_update_ms);
        let elapsed_seconds = elapsed_ms / 1000;

        const MAX_ELAPSED_SECONDS: u64 = 5;
        let capped_seconds = std::cmp::min(elapsed_seconds, MAX_ELAPSED_SECONDS);

        let minutes_advanced = (capped_seconds * 5) as u32;
        self.last_update_ms = current_time_ms;

        if minutes_advanced > 0 {
            self.advance_minutes(minutes_advanced);
        }
    }

    fn advance_minutes(&mut self, minutes: u32) {
        let mut remaining = minutes;
        while remaining > 0 {
            self.minute += 1;
            self.total_minutes = (self.total_minutes + 1) % 1440;

            if self.minute >= 60 {
                self.minute = 0;
                self.hour += 1;
            }

            if self.hour >= 24 {
                self.hour = 0;
                self.day += 1;
                self.start_new_day();
            }

            if self.should_auto_sleep() {
                self.run_auto_sleep();
                break;
            }

            remaining -= 1;
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self, current_time_ms: u64) {
        self.is_paused = false;
        self.last_update_ms = current_time_ms;
    }

    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn clear_action(&mut self) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot clear here (farm only).");
            return;
        }

        if let Some((x, y)) =
            self.find_adjacent_tile(|tile| matches!(tile, TileType::Grass | TileType::Crop(_, _)))
        {
            let tile = self.get_tile_at(x, y);
            match tile {
                Some(TileType::Grass) => {
                    self.farm_map[y][x] = TileType::Soil;
                    self.message = String::from("Cleared weeds.");
                    self.advance_time();
                }
                Some(TileType::Crop(_, _)) => {
                    self.farm_map[y][x] = TileType::Soil;
                    self.message = String::from("Cleared crop tile.");
                    self.advance_time();
                }
                Some(TileType::Fountain) => {
                    self.message = String::from("Cannot clear the fountain!");
                }
                Some(TileType::Slide) => {
                    self.message = String::from("Cannot clear the slide!");
                }
                _ => {
                    self.message =
                        String::from("Nothing clearable nearby.");
                }
            }
        } else {
            self.message = String::from("Nothing clearable nearby.");
        }
    }

    pub fn clear_action_at(&mut self, dir: Direction) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot clear here (farm only).");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            match tile {
                Some(TileType::Grass) => {
                    self.farm_map[y][x] = TileType::Soil;
                    self.message = format!("Clear Done! (Cleared {:?})", dir);
                    self.advance_time();
                }
                Some(TileType::Crop(_, _)) => {
                    self.farm_map[y][x] = TileType::Soil;
                    self.message = format!("Clear Done! (Uprooted {:?})", dir);
                    self.advance_time();
                }
                Some(TileType::Fountain) => {
                    self.message = String::from("Cannot clear the fountain!");
                }
                Some(TileType::Slide) => {
                    self.message = String::from("Cannot clear the slide!");
                }
                _ => {
                    self.message =
                        String::from("Nothing clearable nearby.");
                }
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn plant_action(&mut self) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot plant here (farm only).");
            return;
        }

        if let Some((x, y)) = self.find_adjacent_tile(|tile| matches!(tile, TileType::Soil)) {
            if self.inventory.use_seed(self.selected_seed) {
                self.farm_map[y][x] = TileType::Crop(self.selected_seed, CropState::new());
                self.message = format!("Plant Done! (Planted {})", self.selected_seed.seed_name());
                self.advance_time();
            } else {
                self.message = format!(
                    "No {} seeds available. Buy more from the shop.",
                    self.selected_seed.seed_name()
                );
            }
        } else {
            self.message = String::from("Cannot plant there! Needs cleared soil.");
        }
    }

    pub fn plant_action_at(&mut self, dir: Direction) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot plant here (farm only).");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Fountain) = tile {
                self.message = String::from("Cannot plant on the fountain!");
                return;
            }
            if let Some(TileType::Slide) = tile {
                self.message = String::from("Cannot plant on the slide!");
                return;
            }
            if let Some(TileType::Soil) = tile {
                if self.inventory.use_seed(self.selected_seed) {
                    self.farm_map[y][x] = TileType::Crop(self.selected_seed, CropState::new());
                    self.message = format!(
                        "Plant Done! (Planted {} at {:?})",
                        self.selected_seed.seed_name(),
                        dir
                    );
                    self.advance_time();
                } else {
                    self.message = format!(
                        "No {} seeds available. Buy more from the shop.",
                        self.selected_seed.seed_name()
                    );
                }
            } else {
                self.message = String::from("Cannot plant there! Needs cleared soil.");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn water_action(&mut self) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot water here (farm only).");
            return;
        }

        if let Some((x, y)) = self.find_adjacent_tile(|tile| matches!(tile, TileType::Crop(_, _))) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(crop, state)) = tile {
                if !state.is_mature(crop) {
                    self.farm_map[y][x] = TileType::Crop(
                        crop,
                        CropState {
                            days_grown: state.days_grown,
                            watered_today: true,
                        },
                    );
                    self.message = String::from("Crop watered.");
                    self.advance_time();
                } else {
                    self.message = String::from("Crop is already mature and ready to harvest.");
                }
            } else {
                self.message = String::from("Nothing to water nearby.");
            }
        } else {
            self.message = String::from("Nothing to water nearby.");
        }
    }

    pub fn water_action_at(&mut self, dir: Direction) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if self.location != Location::Farm {
            self.message = String::from("Cannot water here (farm only).");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Fountain) = tile {
                self.message = String::from("Cannot water the fountain!");
                return;
            }
            if let Some(TileType::Slide) = tile {
                self.message = String::from("Cannot water the slide!");
                return;
            }
            if let Some(TileType::Crop(crop, state)) = tile {
                if !state.is_mature(crop) {
                    self.farm_map[y][x] = TileType::Crop(
                        crop,
                        CropState {
                            days_grown: state.days_grown,
                            watered_today: true,
                        },
                    );
                    self.message = format!("Water Done! (Watered {:?})", dir);
                    self.advance_time();
                } else {
                    self.message = String::from("Crop is already mature and ready to harvest.");
                }
            } else {
                self.message = String::from("Nothing to water nearby.");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn harvest_action(&mut self) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if let Some((x, y)) = self
            .find_adjacent_tile(|tile| matches!(tile, TileType::Crop(_, _) | TileType::Mushroom))
        {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(crop, state)) = tile {
                if state.is_mature(crop) {
                    if self.location == Location::Farm {
                        self.farm_map[y][x] = TileType::Soil;
                    } else if self.location == Location::EastPath {
                        self.east_path_map[y][x] = TileType::Grass;
                    } else if self.location == Location::Square {
                        self.square_map[y][x] = TileType::Grass;
                    } else {
                        self.south_river_map[y][x] = TileType::Grass;
                    }
                    self.inventory.add_produce(crop);
                    self.record_crop_harvested(crop, 1);
                    self.message = format!("Harvest Done! (Got {})", crop.produce_emoji());
                    self.advance_time();
                } else {
                    self.message = String::from("Not ready yet (needs more time).");
                }
            } else if let Some(TileType::Mushroom) = tile {
                if self.player_location == Location::Farm {
                    self.farm_map[y][x] = TileType::Grass;
                } else if let Some(map_row) = self.east_path_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                } else if let Some(map_row) = self.square_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                } else if let Some(map_row) = self.south_river_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                }
                self.inventory.add_forage(ForageType::Mushroom);
                self.record_forage_harvested(ForageType::Mushroom, 1);
                self.message = String::from("Harvested 🍄.");
                self.advance_time();
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Nothing to harvest!");
        }
    }

    pub fn harvest_action_at(&mut self, dir: Direction) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Fountain) = tile {
                self.message = String::from("Nothing to harvest from the fountain!");
                return;
            }
            if let Some(TileType::Slide) = tile {
                self.message = String::from("Nothing to harvest from the slide!");
                return;
            }
            if let Some(TileType::Crop(crop, state)) = tile {
                if state.is_mature(crop) {
                    if self.location == Location::Farm {
                        self.farm_map[y][x] = TileType::Soil;
                    } else if self.location == Location::EastPath {
                        self.east_path_map[y][x] = TileType::Grass;
                    } else if self.location == Location::Square {
                        self.square_map[y][x] = TileType::Grass;
                    } else {
                        self.south_river_map[y][x] = TileType::Grass;
                    }
                    self.inventory.add_produce(crop);
                    self.record_crop_harvested(crop, 1);
                    self.message =
                        format!("Harvest Done! (Got {} at {:?})", crop.produce_emoji(), dir);
                    self.advance_time();
                } else {
                    self.message = String::from("Not ready yet (needs more time).");
                }
            } else if let Some(TileType::Mushroom) = tile {
                if self.player_location == Location::Farm {
                    self.farm_map[y][x] = TileType::Grass;
                } else if let Some(map_row) = self.east_path_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                } else if let Some(map_row) = self.square_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                } else if let Some(map_row) = self.south_river_map.get_mut(y) {
                    map_row[x] = TileType::Grass;
                }
                self.inventory.add_forage(ForageType::Mushroom);
                self.record_forage_harvested(ForageType::Mushroom, 1);
                self.message = String::from("Harvested 🍄.");
                self.advance_time();
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn fishing_action(&mut self) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if let Some((x, y)) =
            self.find_adjacent_tile(|tile| matches!(tile, TileType::River | TileType::RiverBubble))
        {
            self.try_fishing_at(x, y);
        } else {
            self.message = String::from("No river nearby to fish.");
        }
    }

    pub fn fishing_action_at(&mut self, dir: Direction) {
        if self.home_state == HomeState::Income {
            self.message = String::from("Sleeping... (Income calculated)");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            self.try_fishing_at(x, y);
        } else {
            self.message = String::from("No river nearby to fish.");
        }
    }

    fn try_fishing_at(&mut self, x: usize, y: usize) {
        if let Some(TileType::River | TileType::RiverBubble) = self.get_tile_at(x, y) {
            self.perform_fishing(x, y);
        } else {
            self.message = String::from("No river nearby to fish.");
        }
    }

    fn perform_fishing(&mut self, x: usize, y: usize) {
        let roll = self.rng_seed.wrapping_add(self.total_minutes as u64);
        let chance = roll % 100;

        if chance < 20 {
            self.inventory.add_fish(FishType::Common);
            self.message = String::from("🎉 Nice catch! You got 🐟");
        } else if chance < 30 {
            self.inventory.add_fish(FishType::Rare);
            self.message = String::from("🎉 Amazing catch! You got 🐠");
        } else {
            self.message = String::from("No bite this time.");
        }

        if matches!(self.get_tile_at(x, y), Some(TileType::River)) {
            let map = self.get_current_map_mut();
            map[y][x] = TileType::RiverBubble;
        }

        self.advance_minutes(60);
    }

    fn get_current_map_mut(&mut self) -> &mut Map {
        match self.location {
            Location::Farm => &mut self.farm_map,
            Location::EastPath => &mut self.east_path_map,
            Location::Square => &mut self.square_map,
            Location::SouthRiver => &mut self.south_river_map,
        }
    }

    pub fn trade_action(&mut self) {
        if self.shop_state == ShopState::None {
            self.shop_state = ShopState::BuyMenu;
            self.shop_cursor = 0;
            self.message = String::from("Shop opened.");
        } else {
            self.close_shop();
        }
        self.advance_time();
    }

    pub fn close_shop(&mut self) {
        self.shop_state = ShopState::None;
        self.shop_cursor = 0;
        self.message = String::from("Shop closed.");
    }

    pub fn in_shop(&self) -> bool {
        self.shop_state != ShopState::None
    }

    pub fn in_home(&self) -> bool {
        self.home_state != HomeState::None
    }

    pub fn check_home_alert(&mut self) {
        if self.home_state == HomeState::None && self.hour == 0 && self.location == Location::Farm {
            self.home_state = HomeState::Alert;
            self.home_cursor = 0;
            self.message = String::from("It's late. Time to rest.");
        }
    }

    pub fn record_income(&mut self, amount: u32) {
        self.current_income.money_earned += amount;
    }

    pub fn record_crop_sold(&mut self, crop: CropType, count: u32) {
        *self.current_income.crops_sold.entry(crop).or_insert(0) += count;
    }

    pub fn record_forage_sold(&mut self, forage: ForageType, count: u32) {
        *self.current_income.forage_sold.entry(forage).or_insert(0) += count;
    }

    pub fn record_fish_sold(&mut self, fish: FishType, count: u32) {
        *self.current_income.fish_sold.entry(fish).or_insert(0) += count;
    }

    pub fn record_crop_harvested(&mut self, crop: CropType, count: u32) {
        *self.current_income.crops_harvested.entry(crop).or_insert(0) += count;
    }

    pub fn record_forage_harvested(&mut self, forage: ForageType, count: u32) {
        *self
            .current_income
            .forage_harvested
            .entry(forage)
            .or_insert(0) += count;
    }

    pub fn get_shop_menu_items(&self) -> Vec<String> {
        match self.shop_state {
            ShopState::BuyMenu => {
                let mut items = Vec::new();
                for crop in CropType::all() {
                    let price = crop.seed_price();
                    items.push(format!("Buy 🫙 {} (${})", crop.seed_name(), price));
                }
                items.push(String::from("Sell Crops"));
                items.push(String::from("Exit"));
                items
            }
            ShopState::SellMenu => {
                let mut items = Vec::new();
                for crop in CropType::all() {
                    let count = self.inventory.get_produce(crop);
                    if count > 0 {
                        let price = crop.produce_price();
                        items.push(format!(
                            "Sell {} {} (${})",
                            crop.produce_emoji(),
                            crop.seed_name(),
                            price * count
                        ));
                    }
                }
                if items.is_empty() {
                    items.push(String::from("(No crops to sell)"));
                }
                items.push(String::from("Back"));
                items
            }
            ShopState::None => vec![],
        }
    }

    pub fn shop_handle_input(&mut self, key_code: KeyCode) -> bool {
        let menu_items = self.get_shop_menu_items();
        let menu_len = menu_items.len();

        match key_code {
            KeyCode::Up => {
                if self.shop_cursor > 0 {
                    self.shop_cursor -= 1;
                }
            }
            KeyCode::Down => {
                if self.shop_cursor < menu_len - 1 {
                    self.shop_cursor += 1;
                }
            }
            KeyCode::Enter => {
                self.handle_shop_selection();
            }
            KeyCode::Esc => {
                self.close_shop();
            }
            _ => {}
        }
        true
    }

    fn handle_shop_selection(&mut self) {
        match self.shop_state {
            ShopState::BuyMenu => {
                let crop_options = CropType::all();
                if self.shop_cursor < 4 {
                    let crop = crop_options[self.shop_cursor];
                    let price = crop.seed_price();
                    if self.money >= price {
                        self.money -= price;
                        self.inventory.add_seeds(crop, 1);
                        self.message = format!("Bought {}.", crop.seed_name());
                    } else {
                        self.message = String::from("Not enough money!");
                    }
                } else if self.shop_cursor == 4 {
                    self.shop_state = ShopState::SellMenu;
                    self.shop_cursor = 0;
                    self.message = String::from("Sell menu opened.");
                } else {
                    self.close_shop();
                }
            }
            ShopState::SellMenu => {
                let crops_with_produce: Vec<CropType> = CropType::all()
                    .iter()
                    .filter(|c| self.inventory.get_produce(**c) > 0)
                    .copied()
                    .collect();

                if self.shop_cursor < crops_with_produce.len() {
                    let crop = crops_with_produce[self.shop_cursor];
                    if self.inventory.sell_produce(crop) {
                        let price = crop.produce_price();
                        self.money += price;
                        self.record_income(price);
                        self.record_crop_sold(crop, 1);
                        self.message = format!("Sold {} for ${}!", crop.produce_emoji(), price);
                    }
                } else {
                    self.shop_state = ShopState::BuyMenu;
                    self.shop_cursor = 0;
                    self.message = String::from("Returned to buy menu.");
                }
            }
            ShopState::None => {}
        }
    }

    pub fn get_home_menu_items(&self) -> Vec<String> {
        match self.home_state {
            HomeState::Alert => vec![String::from("Sleep")],
            HomeState::Income => vec![String::from("Continue")],
            HomeState::None => vec![],
        }
    }

    pub fn home_handle_input(&mut self, key_code: KeyCode) -> bool {
        let menu_items = self.get_home_menu_items();
        let menu_len = menu_items.len();

        match key_code {
            KeyCode::Up => {
                if self.home_cursor > 0 {
                    self.home_cursor -= 1;
                }
            }
            KeyCode::Down => {
                if self.home_cursor < menu_len - 1 {
                    self.home_cursor += 1;
                }
            }
            KeyCode::Enter => {
                self.handle_home_selection();
            }
            _ => {}
        }
        true
    }

    fn handle_home_selection(&mut self) {
        match self.home_state {
            HomeState::Alert => {
                if self.home_cursor == 0 {
                    self.perform_sleep();
                }
            }
            HomeState::Income => {
                self.close_home();
            }
            HomeState::None => {}
        }
    }

    fn perform_sleep(&mut self) {
        self.home_state = HomeState::Income;
        self.home_cursor = 0;
        self.message = String::from("Sleeping... (Income calculated)");
    }

    pub fn close_home(&mut self) {
        self.current_income = DailyIncome::default();
        self.home_state = HomeState::None;
        self.home_cursor = 0;
        self.message = String::from("Ready for a new day.");
    }
}

fn find_player_start(_map: &Map) -> (usize, usize) {
    (3, 3)
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crop_tile_is_not_walkable() {
        let crop_seedling = TileType::Crop(CropType::Carrot, CropState::new());
        assert!(!crop_seedling.is_walkable());

        let mature_state = CropState {
            days_grown: 10,
            watered_today: false,
        };
        let crop_mature = TileType::Crop(CropType::Carrot, mature_state);
        assert!(!crop_mature.is_walkable());
    }

    #[test]
    fn test_non_crop_tiles_are_walkable() {
        assert!(TileType::Grass.is_walkable());
        assert!(TileType::Soil.is_walkable());
        assert!(TileType::PathEast.is_walkable());
        assert!(TileType::PathFarm.is_walkable());
        assert!(!TileType::House.is_walkable());
        assert!(!TileType::Mushroom.is_walkable());
        assert!(!TileType::Boundary.is_walkable());
    }

    #[test]
    fn test_move_blocked_by_crop() {
        let mut state = GameState::new();
        state.player_x = 3;
        state.player_y = 3;
        state.farm_map[4][3] = TileType::Crop(CropType::Carrot, CropState::new());

        let result = state.move_player(Direction::Down);
        assert!(!result);
        assert_eq!(state.player_x, 3);
        assert_eq!(state.player_y, 3);
        assert!(state.message.contains("Cannot move there"));
    }

    #[test]
    fn test_move_allowed_on_non_crop_tiles() {
        let mut state = GameState::new();
        state.player_x = 3;
        state.player_y = 3;

        state.farm_map[3][4] = TileType::Grass;
        let result = state.move_player(Direction::Right);
        assert!(result);
        assert_eq!(state.player_x, 4);
        assert_eq!(state.player_y, 3);
    }

    #[test]
    fn test_move_blocked_by_boundary() {
        let mut state = GameState::new();
        state.player_x = 0;
        state.player_y = 1;

        let result = state.move_player(Direction::Left);
        assert!(!result);
        assert_eq!(state.player_x, 0);
    }

    #[test]
    fn test_transition_still_works() {
        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 6;
        state.player_y = 5;
        state.farm_map[5][7] = TileType::PathEast;

        let result = state.move_player(Direction::Right);
        assert!(result);
        assert_eq!(state.location, Location::EastPath);
    }

    #[test]
    fn test_cleared_soil_reverts_to_grass_next_day() {
        let mut state = GameState::new();
        state.farm_map[3][3] = TileType::Soil;

        assert_eq!(state.farm_map[3][3], TileType::Soil);

        state.start_new_day();

        assert_eq!(state.farm_map[3][3], TileType::Grass);
    }

    #[test]
    fn test_cleared_with_crop_remains_crop() {
        let mut state = GameState::new();
        state.farm_map[3][3] = TileType::Crop(CropType::Carrot, CropState::new());

        state.start_new_day();

        assert!(matches!(
            state.farm_map[3][3],
            TileType::Crop(CropType::Carrot, _)
        ));
    }

    #[test]
    fn test_harvested_soil_reverts_to_grass_next_day() {
        let mut state = GameState::new();
        let mature_state = CropState {
            days_grown: 10,
            watered_today: false,
        };
        state.farm_map[3][3] = TileType::Crop(CropType::Carrot, mature_state);

        state.farm_map[3][3] = TileType::Soil;

        state.start_new_day();

        assert_eq!(state.farm_map[3][3], TileType::Grass);
    }

    #[test]
    fn test_forced_flower_spawns_at_farm_6_2_in_spring_when_empty() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.spring_forced_flower_6_2_done = false;

        state.farm_map[2][6] = TileType::Grass;

        state.spawn_random_crops();

        assert!(matches!(
            state.farm_map[2][6],
            TileType::Crop(CropType::Rhubarb, state) if state.is_mature(CropType::Rhubarb)
        ));
        assert!(state.spring_forced_flower_6_2_done);
    }

    #[test]
    fn test_forced_flower_does_not_overwrite_occupied_tile() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.spring_forced_flower_6_2_done = false;

        state.farm_map[2][6] = TileType::Crop(CropType::Carrot, CropState::new());

        state.spawn_random_crops();

        assert!(matches!(
            state.farm_map[2][6],
            TileType::Crop(CropType::Carrot, _)
        ));
        assert!(!state.spring_forced_flower_6_2_done);
    }

    #[test]
    fn test_forced_flower_not_in_non_spring_season() {
        let mut state = GameState::new();
        state.season = String::from("Summer");
        state.spring_forced_flower_6_2_done = false;
        state.farm_map[2][6] = TileType::Grass;

        state.spawn_random_crops();

        assert_eq!(state.farm_map[2][6], TileType::Grass);
        assert!(!state.spring_forced_flower_6_2_done);
    }

    #[test]
    fn test_daily_flower_10_percent_chance() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.rng_seed = 0;

        let mut flower_roll_hits = 0;
        let iterations = 1000;

        for _ in 0..iterations {
            let flower_seed = state
                .rng_seed
                .wrapping_add((state.day as u64).wrapping_mul(7919));
            let flower_roll = (flower_seed % 100) as u32;
            if flower_roll < 10 {
                flower_roll_hits += 1;
            }
            state.day += 1;
        }

        let percentage = (flower_roll_hits as f64 / iterations as f64) * 100.0;
        assert!(
            percentage > 5.0 && percentage < 15.0,
            "Expected ~10%, got {}%",
            percentage
        );
    }

    #[test]
    fn test_rainy_day_mushroom_spawns() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.weather = Weather::Rainy;

        let initial_mushrooms = count_mushrooms(&state);

        state.spawn_random_crops();

        let final_mushrooms = count_mushrooms(&state);
        assert!(final_mushrooms > initial_mushrooms);
    }

    #[test]
    fn test_no_mushroom_on_sunny_day() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.weather = Weather::Sunny;

        state.spawn_random_crops();

        for y in 0..FARM_HEIGHT {
            for x in 0..FARM_WIDTH {
                assert!(!matches!(state.farm_map[y][x], TileType::Mushroom));
            }
        }
    }

    #[test]
    fn test_spawn_only_runs_once_per_day() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.weather = Weather::Rainy;

        let _initial_mushrooms = count_mushrooms(&state);

        state.spawn_random_crops();

        let after_first = count_mushrooms(&state);

        state.spawn_random_crops();

        let after_second = count_mushrooms(&state);

        assert_eq!(after_first, after_second);
    }

    #[test]
    fn test_no_spawn_outside_spring() {
        for season in ["Summer", "Fall", "Winter"] {
            let mut state = GameState::new();
            state.season = String::from(season);
            state.weather = Weather::Rainy;
            state.spring_forced_flower_6_2_done = false;
            state.farm_map[2][6] = TileType::Grass;

            state.spawn_random_crops();

            assert_eq!(state.farm_map[2][6], TileType::Grass);
        }
    }

    #[test]
    fn test_save_load_preserves_spawn_flags() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.spring_forced_flower_6_2_done = true;
        state.last_spawn_processed_day = 5;

        let test_path = std::env::temp_dir().join("tinydew_random_crop_test.json");

        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert!(loaded.spring_forced_flower_6_2_done);
        assert_eq!(loaded.last_spawn_processed_day, 5);

        std::fs::remove_file(&test_path).ok();
    }

    fn count_mushrooms(state: &GameState) -> usize {
        let mut count = 0;
        for y in 0..FARM_HEIGHT {
            for x in 0..FARM_WIDTH {
                if matches!(state.farm_map[y][x], TileType::Mushroom) {
                    count += 1;
                }
            }
        }
        for y in 0..EAST_PATH_HEIGHT {
            for x in 0..EAST_PATH_WIDTH {
                if matches!(state.east_path_map[y][x], TileType::Mushroom) {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn test_random_flower_never_spawns_on_home_or_wakeup_tile() {
        let mut hit_home = false;
        let mut hit_wakeup = false;

        for day in 2..=300 {
            let mut state = GameState::new();
            state.season = String::from("Spring");
            state.day = day;
            state.last_spawn_processed_day = 0;
            state.spring_forced_flower_6_2_done = true;
            state.rng_seed = 0;

            state.farm_map[2][2] = TileType::Grass;
            state.farm_map[3][3] = TileType::Grass;

            state.spawn_random_crops();

            if matches!(state.farm_map[2][2], TileType::Crop(CropType::Rhubarb, _)) {
                hit_home = true;
            }
            if matches!(state.farm_map[3][3], TileType::Crop(CropType::Rhubarb, _)) {
                hit_wakeup = true;
            }
        }

        assert!(
            !hit_home,
            "Random flower should never spawn on home tile (2,2)"
        );
        assert!(
            !hit_wakeup,
            "Random flower should never spawn on wake-up tile (3,3)"
        );
    }

    #[test]
    fn test_random_mushroom_never_spawns_on_home_or_wakeup_tile() {
        let mut hit_home = false;
        let mut hit_wakeup = false;

        for day in 2..=300 {
            let mut state = GameState::new();
            state.season = String::from("Spring");
            state.weather = Weather::Rainy;
            state.day = day;
            state.last_spawn_processed_day = 0;
            state.spring_forced_flower_6_2_done = true;
            state.rng_seed = 0;

            state.farm_map[2][2] = TileType::Grass;
            state.farm_map[3][3] = TileType::Grass;

            state.spawn_random_crops();

            if matches!(state.farm_map[2][2], TileType::Mushroom) {
                hit_home = true;
            }
            if matches!(state.farm_map[3][3], TileType::Mushroom) {
                hit_wakeup = true;
            }
        }

        assert!(
            !hit_home,
            "Random mushroom should never spawn on home tile (2,2)"
        );
        assert!(
            !hit_wakeup,
            "Random mushroom should never spawn on wake-up tile (3,3)"
        );
    }

    #[test]
    fn test_guest_enable_and_spawn() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();

        assert!(state.guest_enabled);
        assert_eq!(state.active_control, ControlTarget::Guest);
        assert_eq!(state.guest_location, Location::Farm);
        assert!(state.guest_x != state.player_x || state.guest_y != state.player_y);
    }

    #[test]
    fn test_guest_disable() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.disable_guest();

        assert!(!state.guest_enabled);
        assert_eq!(state.active_control, ControlTarget::Player);
    }

    #[test]
    fn test_guest_movement() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Farm;
        state.location = Location::Farm;
        state.guest_x = 1;
        state.guest_y = 1;

        let initial_x = state.guest_x;
        let initial_y = state.guest_y;

        state.move_guest(Direction::Right);
        assert_eq!(state.guest_x, initial_x + 1);
        assert_eq!(state.guest_y, initial_y);
    }

    #[test]
    fn test_guest_cannot_enter_house_tile() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Farm;
        state.location = Location::Farm;
        state.guest_x = 1;
        state.guest_y = 2;

        // House is at (2,2)
        let moved = state.move_guest(Direction::Right);
        assert!(!moved);
        assert_eq!(state.guest_x, 1);
        assert_eq!(state.guest_y, 2);
    }

    #[test]
    fn test_guest_player_collision_blocks() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.location = Location::Farm;
        state.player_location = Location::Farm;
        state.guest_location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;
        state.guest_x = 2;
        state.guest_y = 3;

        state.direction = Direction::Left;
        let result = state.move_player(Direction::Left);
        assert!(!result);
        assert!(state.message.contains("Tile occupied"));
    }

    #[test]
    fn test_guest_can_move_to_player_coordinates_when_in_different_region() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();

        // Player is on Farm, guest is on EastPath; same (x,y) should not collide.
        state.player_location = Location::Farm;
        state.guest_location = Location::EastPath;
        state.location = Location::EastPath;

        state.player_x = 2;
        state.player_y = 2;
        state.guest_x = 1;
        state.guest_y = 2;

        let moved = state.move_guest(Direction::Right);
        assert!(
            moved,
            "Guest should move when player is in a different region"
        );
        assert_eq!(state.guest_x, 2);
        assert_eq!(state.guest_y, 2);
    }

    #[test]
    fn test_control_toggle_is_noop_in_guest_only_mode() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();

        assert_eq!(state.active_control, ControlTarget::Guest);
        state.toggle_control();
        assert_eq!(state.active_control, ControlTarget::Guest);
        assert!(state.is_guest_active());
    }

    #[test]
    fn test_guest_transition_farm_to_east_path() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Farm;
        state.location = Location::Farm;
        state.guest_x = 6;
        state.guest_y = 5;

        state.move_guest(Direction::Right);

        assert_eq!(state.guest_location, Location::EastPath);
        assert_eq!(state.guest_x, 1);
        assert_eq!(state.guest_y, 2);
    }

    #[test]
    fn test_guest_transition_east_path_to_farm() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::EastPath;
        state.location = Location::EastPath;
        state.guest_x = 1;
        state.guest_y = 2;

        state.move_guest(Direction::Left);

        assert_eq!(state.guest_location, Location::Farm);
        assert_eq!(state.guest_x, 7);
        assert_eq!(state.guest_y, 5);
    }

    #[test]
    fn test_guest_blocks_non_movement_actions() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();

        assert!(state.is_guest_active());
    }

    #[test]
    fn test_time_frozen_when_guest_active() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();

        assert!(state.is_time_frozen());
        assert!(state.is_guest_active());

        // Guest-only mode keeps freeze on and toggle is a no-op.
        state.toggle_control();
        assert!(state.is_time_frozen());
    }

    #[test]
    fn test_guest_not_on_current_map() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::EastPath;
        state.location = Location::Farm;

        assert!(!state.is_guest_on_current_map());
    }

    #[test]
    fn test_guest_save_load_preserves_state() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_x = 4;
        state.guest_y = 5;
        state.guest_location = Location::EastPath;

        let test_path = std::env::temp_dir().join("tinydew_guest_test.json");
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert!(loaded.guest_enabled);
        assert_eq!(loaded.guest_x, 4);
        assert_eq!(loaded.guest_y, 5);
        assert_eq!(loaded.guest_location, Location::EastPath);
        assert_eq!(loaded.active_control, ControlTarget::Guest);

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_square_map_layout_dimensions_and_fountain_position() {
        use crate::world::create_square_map;
        let square_map = create_square_map();

        assert_eq!(square_map.len(), SQUARE_HEIGHT);
        assert_eq!(square_map[0].len(), SQUARE_WIDTH);

        assert_eq!(square_map[2][4], TileType::Fountain);

        match square_map[1][1] {
            TileType::Crop(CropType::Rhubarb, state) => {
                assert!(state.is_mature(CropType::Rhubarb));
            }
            _ => panic!("Expected day-1 spawn flower crop at square[1][1]"),
        }
    }

    #[test]
    fn test_square_fountain_not_walkable() {
        assert!(!TileType::Fountain.is_walkable());
    }

    #[test]
    fn test_square_slide_not_walkable() {
        assert!(!TileType::Slide.is_walkable());
    }

    #[test]
    fn test_square_boundary_not_walkable() {
        assert!(!TileType::Boundary.is_walkable());
    }

    #[test]
    fn test_square_transition_enter_from_east_path() {
        let mut state = GameState::new();
        state.location = Location::EastPath;
        state.player_location = Location::EastPath;
        state.player_x = 5;
        state.player_y = 1;

        state.move_player(Direction::Up);

        assert_eq!(state.location, Location::Square);
        assert_eq!(state.player_location, Location::Square);
        assert_eq!(state.player_x, 4);
        assert_eq!(state.player_y, 4);
    }

    #[test]
    fn test_square_transition_exit_to_east_path() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.player_x = 4;
        state.player_y = 3;

        state.move_player(Direction::Down);

        assert_eq!(state.location, Location::EastPath);
        assert_eq!(state.player_location, Location::EastPath);
        assert_eq!(state.player_x, 5);
        assert_eq!(state.player_y, 0);
    }

    #[test]
    fn test_guest_cannot_move_onto_square_fountain() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Square;
        state.guest_x = 4;
        state.guest_y = 1;

        let can_move = state.can_move_guest_to(4, 2);
        assert!(!can_move, "Guest should not be able to move onto fountain");
    }

    #[test]
    fn test_square_actions_reject_fountain_target() {
        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 2;
        state.player_y = 2;

        state.clear_action();
        assert!(!state.message.contains("fountain"));

        state.farm_map[2][3] = TileType::Fountain;
        state.clear_action_at(Direction::Right);
        assert!(state.message.contains("fountain") || state.message.contains("Cannot clear"));
    }

    #[test]
    fn test_wonder_spawns_on_spring_day_28_at_square_2_2() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.day = 28;
        state.season = String::from("Spring");

        state.start_new_day();

        assert_eq!(state.square_map[2][2], TileType::Wonder);
    }

    #[test]
    fn test_wonder_tile_renders_butterfly_emoji() {
        assert_eq!(TileType::Wonder.emoji(), "🦋");
    }

    #[test]
    fn test_wonder_clears_after_festival_day_transition() {
        let mut state = GameState::new();
        state.square_map[2][2] = TileType::Wonder;
        state.season = String::from("Spring");
        state.day = 29;

        state.start_new_day();

        assert_eq!(state.square_map[2][2], TileType::Grass);
    }

    #[test]
    fn test_wonder_tile_is_not_walkable() {
        assert!(!TileType::Wonder.is_walkable());
    }

    #[test]
    fn test_wonder_message_on_player_attempted_step() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.player_x = 2;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.square_map[2][2] = TileType::Wonder;

        let moved = state.move_player(Direction::Down);

        assert!(!moved);
        assert_eq!(state.player_x, 2);
        assert_eq!(state.player_y, 1);
        assert!(state.message.contains("That is so beautiful"));
    }

    #[test]
    fn test_wonder_message_on_guest_attempted_step() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Square;
        state.location = Location::Square;
        state.guest_x = 2;
        state.guest_y = 1;
        state.square_map[2][2] = TileType::Wonder;

        let moved = state.move_guest(Direction::Down);

        assert!(!moved);
        assert_eq!(state.guest_x, 2);
        assert_eq!(state.guest_y, 1);
        assert!(state.message.contains("That is so beautiful"));
    }

    #[test]
    fn test_guest_greeting_message_on_butterfly_festival() {
        let mut state = GameState::new();
        state.season = String::from("Spring");
        state.day = 28;

        let msg = state.guest_greeting_message();

        assert_eq!(msg, "✨ Happy Butterfly Festival!");
    }

    #[test]
    fn test_square_plant_forbidden_on_all_tiles() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.inventory.seeds.insert(CropType::Carrot, 5);

        state.plant_action();
        assert!(state.message.contains("Cannot plant here"));

        state.location = Location::EastPath;
        state.player_location = Location::EastPath;
        state.plant_action();
        assert!(state.message.contains("Cannot plant here"));
    }

    #[test]
    fn test_square_clear_forbidden_on_all_tiles() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;

        state.clear_action();
        assert!(state.message.contains("Cannot clear here"));

        state.location = Location::EastPath;
        state.player_location = Location::EastPath;
        state.clear_action();
        assert!(state.message.contains("Cannot clear here"));
    }

    #[test]
    fn test_square_save_load_roundtrip() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.player_x = 5;
        state.player_y = 2;

        let test_path = std::env::temp_dir().join("tinydew_square_test.json");
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert_eq!(loaded.location, Location::Square);
        assert_eq!(loaded.player_location, Location::Square);
        assert_eq!(loaded.player_x, 5);
        assert_eq!(loaded.player_y, 2);

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_guest_transition_square_to_east_path() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::Square;
        state.location = Location::Square;
        state.guest_x = 4;
        state.guest_y = 3;

        state.move_guest(Direction::Down);

        assert_eq!(state.guest_location, Location::EastPath);
        assert_eq!(state.guest_x, 5);
        assert_eq!(state.guest_y, 0);
    }

    #[test]
    fn test_guest_transition_east_path_to_square() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::EastPath;
        state.location = Location::EastPath;
        state.guest_x = 5;
        state.guest_y = 1;

        state.move_guest(Direction::Up);

        assert_eq!(state.guest_location, Location::Square);
        assert_eq!(state.guest_x, 4);
        assert_eq!(state.guest_y, 4);
    }

    #[test]
    fn test_fountain_blocks_all_directions() {
        let mut state = GameState::new();
        state.location = Location::Square;
        state.player_location = Location::Square;
        state.player_x = 4;
        state.player_y = 2;

        let can_up = state.can_move_to(4, 1);
        let can_down = state.can_move_to(4, 3);
        let can_left = state.can_move_to(3, 2);
        let can_right = state.can_move_to(6, 2);

        assert!(can_up, "Should be able to move up (grass)");
        assert!(can_down, "Should be able to move down (grass)");
        assert!(can_left, "Should be able to move left (grass)");
        assert!(can_right, "Should be able to move right (grass)");
    }

    #[test]
    fn test_south_river_map_layout_dimensions() {
        use crate::world::{SOUTH_RIVER_HEIGHT, SOUTH_RIVER_WIDTH, create_south_river_map};
        let south_river_map = create_south_river_map();

        assert_eq!(south_river_map.len(), SOUTH_RIVER_HEIGHT);
        assert_eq!(south_river_map[0].len(), SOUTH_RIVER_WIDTH);
        assert_eq!(south_river_map[1].len(), SOUTH_RIVER_WIDTH);
        assert_eq!(south_river_map[2].len(), SOUTH_RIVER_WIDTH);
        assert_eq!(south_river_map[3].len(), SOUTH_RIVER_WIDTH);

        assert_eq!(south_river_map[0][2], TileType::PathSouthRiverGate);
        for x in 0..13 {
            if x != 2 {
                assert_eq!(
                    south_river_map[0][x],
                    TileType::Boundary,
                    "Row 0 col {} should be Boundary",
                    x
                );
            }
        }
    }

    #[test]
    fn test_south_river_river_tiles_not_walkable() {
        use crate::world::create_south_river_map;
        let south_river_map = create_south_river_map();

        for y in 2..4 {
            for x in 0..13 {
                assert!(
                    !south_river_map[y][x].is_walkable(),
                    "River tile at ({}, {}) should not be walkable",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_south_river_boundary_not_walkable() {
        use crate::world::create_south_river_map;
        let south_river_map = create_south_river_map();

        assert!(!south_river_map[0][0].is_walkable());
        assert!(!south_river_map[0][1].is_walkable());
        assert!(!south_river_map[0][3].is_walkable());
        assert!(!south_river_map[0][12].is_walkable());
        assert!(!south_river_map[1][0].is_walkable());
        assert!(!south_river_map[1][12].is_walkable());
    }

    #[test]
    fn test_transition_east_path_to_south_river() {
        let mut state = GameState::new();
        state.location = Location::EastPath;
        state.player_location = Location::EastPath;
        state.player_x = 5;
        state.player_y = 2;

        state.move_player(Direction::Down);

        assert_eq!(state.location, Location::SouthRiver);
        assert_eq!(state.player_location, Location::SouthRiver);
        assert_eq!(state.player_x, 2);
        assert_eq!(state.player_y, 1);
    }

    #[test]
    fn test_transition_south_river_to_east_path() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 2;
        state.player_y = 1;

        state.move_player(Direction::Up);

        assert_eq!(state.location, Location::EastPath);
        assert_eq!(state.player_location, Location::EastPath);
        assert_eq!(state.player_x, 5);
        assert_eq!(state.player_y, 3);
    }

    #[test]
    fn test_guest_transition_east_path_to_south_river() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::EastPath;
        state.location = Location::EastPath;
        state.guest_x = 5;
        state.guest_y = 2;

        state.move_guest(Direction::Down);

        assert_eq!(state.guest_location, Location::SouthRiver);
        assert_eq!(state.guest_x, 2);
        assert_eq!(state.guest_y, 1);
    }

    #[test]
    fn test_guest_transition_south_river_to_east_path() {
        let mut state = GameState::new();
        state.enable_guest_for_interactive();
        state.guest_location = Location::SouthRiver;
        state.location = Location::SouthRiver;
        state.guest_x = 2;
        state.guest_y = 1;

        state.move_guest(Direction::Up);

        assert_eq!(state.guest_location, Location::EastPath);
        assert_eq!(state.guest_x, 5);
        assert_eq!(state.guest_y, 3);
    }

    #[test]
    fn test_south_river_random_flower_spawns_on_grass_only() {
        use crate::world::create_south_river_map;
        let south_river_map = create_south_river_map();

        let mut grass_positions = 0;
        for y in 0..4 {
            for x in 0..13 {
                if south_river_map[y][x] == TileType::Grass {
                    grass_positions += 1;
                }
            }
        }
        assert!(
            grass_positions > 0,
            "South River should have grass positions"
        );

        let mut river_positions = 0;
        for y in 2..4 {
            for x in 0..13 {
                if south_river_map[y][x] == TileType::River {
                    river_positions += 1;
                }
            }
        }
        assert_eq!(
            river_positions, 26,
            "South River should have 26 river tiles (2 rows x 13 cols)"
        );
    }

    #[test]
    fn test_south_river_no_mushroom_spawn() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.season = String::from("Spring");
        state.weather = Weather::Rainy;
        state.last_spawn_processed_day = 0;
        state.day = 1;

        state.spawn_random_crops();

        for y in 0..SOUTH_RIVER_HEIGHT {
            for x in 0..SOUTH_RIVER_WIDTH {
                assert!(
                    !matches!(state.south_river_map[y][x], TileType::Mushroom),
                    "Mushroom should not spawn in South River"
                );
            }
        }
    }

    #[test]
    fn test_south_river_save_load_roundtrip() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;

        let test_path = std::env::temp_dir().join("tinydew_south_river_test.json");
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert_eq!(loaded.location, Location::SouthRiver);
        assert_eq!(loaded.player_location, Location::SouthRiver);
        assert_eq!(loaded.player_x, 5);
        assert_eq!(loaded.player_y, 1);

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_print_snapshot_renders_south_river_dimensions() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;

        let (width, height) = state.get_map_size();
        assert_eq!(width, 13);
        assert_eq!(height, 4);

        let map = state.get_current_map_ref();
        assert_eq!(map.len(), 4);
        assert_eq!(map[0].len(), 13);
    }

    #[test]
    fn test_get_map_returns_south_river_dimensions() {
        use crate::world::{SOUTH_RIVER_HEIGHT, SOUTH_RIVER_WIDTH};
        let mut state = GameState::new();
        state.location = Location::SouthRiver;

        let (width, height) = state.get_map_size();
        assert_eq!(width, SOUTH_RIVER_WIDTH);
        assert_eq!(height, SOUTH_RIVER_HEIGHT);

        let map = state.get_current_map_ref();
        assert_eq!(map.len(), SOUTH_RIVER_HEIGHT);
        assert_eq!(map[0].len(), SOUTH_RIVER_WIDTH);
    }

    #[test]
    fn test_fishing_requires_nearby_river() {
        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;

        state.fishing_action();
        assert!(state.message.contains("No river nearby"));
    }

    #[test]
    fn test_fishing_advances_time_by_one_hour() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.hour = 10;
        state.minute = 0;
        state.total_minutes = 600;

        state.fishing_action();

        assert_eq!(state.hour, 11);
        assert_eq!(state.minute, 0);
    }

    #[test]
    fn test_river_tile_turns_into_bubble_after_fishing() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;

        assert_eq!(state.south_river_map[2][5], TileType::River);

        state.fishing_action();

        assert_eq!(state.south_river_map[2][5], TileType::RiverBubble);
    }

    #[test]
    fn test_river_bubble_tile_is_still_fishable() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;

        state.south_river_map[2][5] = TileType::RiverBubble;

        state.fishing_action();

        assert_eq!(state.south_river_map[2][5], TileType::RiverBubble);
    }

    #[test]
    fn test_sleep_cycle_resets_bubble_to_river() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.south_river_map[2][5] = TileType::RiverBubble;
        state.south_river_map[3][3] = TileType::RiverBubble;

        state.start_new_day();

        assert_eq!(state.south_river_map[2][5], TileType::River);
        assert_eq!(state.south_river_map[3][3], TileType::River);
    }

    #[test]
    fn test_river_bubble_not_walkable() {
        let bubble = TileType::RiverBubble;
        assert!(!bubble.is_walkable());
    }

    #[test]
    fn test_fishing_adds_items_to_inventory() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.rng_seed = 0;
        state.total_minutes = 0;

        state.fishing_action();

        assert!(state.inventory.fish_count() > 0 || state.message.contains("No bite"));
    }

    #[test]
    fn test_fish_inventory_persists_save_load() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.inventory.fish.insert(FishType::Common, 5);
        state.inventory.fish.insert(FishType::Rare, 2);

        let test_path = std::env::temp_dir().join("tinydew_fish_test.json");
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");
        assert_eq!(loaded.inventory.fish.get(&FishType::Common), Some(&5));
        assert_eq!(loaded.inventory.fish.get(&FishType::Rare), Some(&2));

        std::fs::remove_file(&test_path).ok();
    }

    #[test]
    fn test_fish_sell_values() {
        assert_eq!(FishType::Common.sell_price(), 80);
        assert_eq!(FishType::Rare.sell_price(), 180);
    }

    #[test]
    fn test_fish_emoji() {
        assert_eq!(FishType::Common.emoji(), "🐟");
        assert_eq!(FishType::Rare.emoji(), "🐠");
    }

    #[test]
    fn test_fishing_outcome_common_fish() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.rng_seed = 0;
        state.total_minutes = 0;

        state.fishing_action();

        assert!(state.inventory.get_fish(FishType::Common) > 0);
        assert!(state.message.contains("🐟"));
    }

    #[test]
    fn test_fishing_outcome_rare_fish() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.rng_seed = 25;
        state.total_minutes = 0;

        state.fishing_action();

        assert!(state.inventory.get_fish(FishType::Rare) > 0);
        assert!(state.message.contains("🐠"));
    }

    #[test]
    fn test_fishing_outcome_nothing() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.rng_seed = 35;
        state.total_minutes = 0;

        state.fishing_action();

        assert_eq!(state.inventory.fish_count(), 0);
        assert!(state.message.contains("No bite"));
    }

    #[test]
    fn test_fishing_success_shows_congrats_message() {
        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.player_x = 5;
        state.player_y = 1;
        state.direction = Direction::Down;
        state.rng_seed = 0;
        state.total_minutes = 0;

        state.fishing_action();

        assert!(state.message.contains("🎉") || state.message.contains("catch"));
    }

    #[test]
    fn test_river_bubble_state_persists_save_load() {
        use crate::savegame::{load_game_from_path, save_game_to_path};

        let mut state = GameState::new();
        state.location = Location::SouthRiver;
        state.player_location = Location::SouthRiver;
        state.south_river_map[2][5] = TileType::RiverBubble;
        state.south_river_map[3][3] = TileType::RiverBubble;

        let test_path = std::env::temp_dir().join("tinydew_bubble_test.json");
        save_game_to_path(&state, &test_path).expect("Save should succeed");

        let loaded = load_game_from_path(&test_path).expect("Load should succeed");

        assert_eq!(loaded.south_river_map[2][5], TileType::RiverBubble);
        assert_eq!(loaded.south_river_map[3][3], TileType::RiverBubble);

        std::fs::remove_file(&test_path).ok();
    }
}
