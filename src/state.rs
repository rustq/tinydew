use crate::world::{
    CropState, CropType, Direction, EAST_PATH_HEIGHT, EAST_PATH_WIDTH, FARM_HEIGHT, FARM_WIDTH,
    ForageType, Map, TileType, Weather, create_east_path_map, create_farm_map,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub seeds: HashMap<CropType, u32>,
    pub produce: HashMap<CropType, u32>,
    pub forage: HashMap<ForageType, u32>,
}

#[allow(dead_code)]
impl Inventory {
    pub fn new() -> Self {
        Self {
            seeds: HashMap::new(),
            produce: HashMap::new(),
            forage: HashMap::new(),
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
    pub farm_map: Map,
    pub east_path_map: Map,
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
}

#[allow(dead_code)]
impl GameState {
    pub fn new() -> Self {
        let farm_map = create_farm_map();
        let (player_x, player_y) = find_player_start(&farm_map);

        let east_path_map = create_east_path_map();

        Self {
            location: Location::Farm,
            farm_map,
            east_path_map,
            player_x,
            player_y,
            direction: Direction::Down,
            message: String::from("Welcome to Shelldew!"),
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
        }
    }

    pub fn get_current_map_ref(&self) -> &Map {
        match self.location {
            Location::Farm => &self.farm_map,
            Location::EastPath => &self.east_path_map,
        }
    }

    pub fn get_current_map(&mut self) -> &mut Map {
        match self.location {
            Location::Farm => &mut self.farm_map,
            Location::EastPath => &mut self.east_path_map,
        }
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        match self.location {
            Location::Farm => (FARM_WIDTH, FARM_HEIGHT),
            Location::EastPath => (EAST_PATH_WIDTH, EAST_PATH_HEIGHT),
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
        self.direction = direction;

        let (dx, dy) = direction.delta();
        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if self.can_move_to(new_x, new_y) {
            let target_tile = self.get_tile_at(new_x, new_y);

            if let Some(tile) = target_tile {
                if tile.is_transition() {
                    self.handle_transition(&tile);
                } else {
                    self.player_x = new_x;
                    self.player_y = new_y;
                    self.advance_time();
                }
            }
            true
        } else {
            self.message = String::from("Cannot move there!");
            false
        }
    }

    fn handle_transition(&mut self, tile: &TileType) {
        match (self.location, tile) {
            (Location::Farm, TileType::PathEast) => {
                self.location = Location::EastPath;
                self.player_x = 1;
                self.player_y = 2;
                self.direction = Direction::Right;
                self.message = String::from("Welcome to East Path!");
            }
            (Location::EastPath, TileType::PathFarm) => {
                self.location = Location::Farm;
                self.player_x = 7;
                self.player_y = 5;
                self.direction = Direction::Left;
                self.message = String::from("Back at the farm!");
            }
            _ => {}
        }
        self.advance_time();
    }

    pub fn advance_time(&mut self) {
        let was_night = self.is_night();

        self.total_minutes += 5;
        self.sync_time_from_minutes();

        if was_night && self.is_day() {
            self.start_new_day();
        }

        if self.should_auto_sleep() {
            self.run_auto_sleep();
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

        self.spawn_east_path_mushrooms();

        self.message = String::from("Good morning! A new day begins.");
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

    fn roll_weather(&mut self) {
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
        let day = self.total_minutes / 1440 + 1;
        let minute_of_day = self.total_minutes % 1440;
        let hour = (minute_of_day / 60) as u8;
        let minute = (minute_of_day % 60) as u8;
        (day, hour, minute)
    }

    pub fn should_auto_sleep(&self) -> bool {
        self.hour == 2
            && self.minute == 0
            && self.auto_sleep_triggered_day != self.day
            && self.home_state == HomeState::None
    }

    pub fn run_auto_sleep(&mut self) {
        self.auto_sleep_triggered_day = self.day;
        self.perform_sleep();
    }

    pub fn run_auto_sleep_and_advance(&mut self) {
        self.day += 1;
        self.auto_sleep_triggered_day = self.day;
        self.hour = 6;
        self.minute = 0;
        self.total_minutes = (self.day - 1) * 1440 + 360;

        self.location = Location::Farm;
        self.player_x = 3;
        self.player_y = 3;

        self.start_new_day();

        self.home_state = HomeState::None;
        self.current_income = DailyIncome::default();
        self.message = String::from("Good morning! Ready for another day.");
    }

    pub fn tick(&mut self, current_time_ms: u64) {
        if self.is_paused {
            return;
        }

        let elapsed_ms = current_time_ms.saturating_sub(self.last_update_ms);
        let elapsed_seconds = elapsed_ms / 1000;

        const MAX_ELAPSED_SECONDS: u64 = 5;
        let capped_seconds = std::cmp::min(elapsed_seconds, MAX_ELAPSED_SECONDS);

        let minutes_advanced = capped_seconds * 5;
        self.total_minutes += minutes_advanced as u32;
        self.last_update_ms = current_time_ms;

        self.sync_time_from_minutes();

        if self.should_auto_sleep() {
            self.run_auto_sleep();
        }
    }

    fn sync_time_from_minutes(&mut self) {
        self.day = self.total_minutes / 1440 + 1;
        let minute_of_day = self.total_minutes % 1440;
        self.hour = minute_of_day / 60;
        self.minute = minute_of_day % 60;
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
        if self.location != Location::Farm {
            self.message = String::from("Cannot clear here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Grass) = tile {
                self.farm_map[y][x] = TileType::Soil;
                self.message = String::from("Clear Done! (Weeds cleared)");
                self.advance_time();
            } else {
                self.message = String::from("Nothing to clear! (Only weeds can be cleared)");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn clear_action_at(&mut self, dir: Direction) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot clear here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Grass) = tile {
                self.farm_map[y][x] = TileType::Soil;
                self.message = format!("Clear Done! (Cleared {:?})", dir);
                self.advance_time();
            } else {
                self.message = String::from("Nothing to clear! (Only weeds can be cleared)");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn plant_action(&mut self) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot plant here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Soil) = tile {
                if self.inventory.use_seed(self.selected_seed) {
                    self.farm_map[y][x] = TileType::Crop(self.selected_seed, CropState::new());
                    self.message =
                        format!("Plant Done! (Planted {})", self.selected_seed.seed_name());
                    self.advance_time();
                } else {
                    self.message = format!(
                        "No {} seeds! Press T to buy seeds.",
                        self.selected_seed.seed_name()
                    );
                }
            } else {
                self.message = String::from("Cannot plant there! (Need cleared soil)");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn plant_action_at(&mut self, dir: Direction) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot plant here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
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
                        "No {} seeds! Press T to buy seeds.",
                        self.selected_seed.seed_name()
                    );
                }
            } else {
                self.message = String::from("Cannot plant there! (Need cleared soil)");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn water_action(&mut self) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot water here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
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
                    self.message = String::from("Water Done! (Crop watered)");
                    self.advance_time();
                } else {
                    self.message = String::from("Already mature! (Harvest ready)");
                }
            } else {
                self.message = String::from("Nothing to water! (Need a crop)");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn water_action_at(&mut self, dir: Direction) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot water here! (Farming only on farm)");
            return;
        }

        if let Some((x, y)) = self.tile_at_direction(dir) {
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
                    self.message = format!("Water Done! (Watered {:?})", dir);
                    self.advance_time();
                } else {
                    self.message = String::from("Already mature! (Harvest ready)");
                }
            } else {
                self.message = String::from("Nothing to water! (Need a crop)");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn harvest_action(&mut self) {
        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(crop, state)) = tile {
                if state.is_mature(crop) {
                    if self.location == Location::Farm {
                        self.farm_map[y][x] = TileType::Soil;
                    } else {
                        self.east_path_map[y][x] = TileType::Grass;
                    }
                    self.inventory.add_produce(crop);
                    self.message = format!("Harvest Done! (Got {})", crop.produce_emoji());
                    self.advance_time();
                } else {
                    self.message = String::from("Not ready yet! (Needs more time)");
                }
            } else if let Some(TileType::Mushroom) = tile {
                if self.location == Location::EastPath {
                    if let Some(map_row) = self.east_path_map.get_mut(y) {
                        map_row[x] = TileType::Grass;
                    }
                    self.inventory.add_forage(ForageType::Mushroom);
                    self.message = String::from("Harvest Done! (Got 🍄)");
                    self.advance_time();
                } else {
                    self.message = String::from("Cannot harvest mushrooms here!");
                }
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn harvest_action_at(&mut self, dir: Direction) {
        if let Some((x, y)) = self.tile_at_direction(dir) {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(crop, state)) = tile {
                if state.is_mature(crop) {
                    if self.location == Location::Farm {
                        self.farm_map[y][x] = TileType::Soil;
                    } else {
                        self.east_path_map[y][x] = TileType::Grass;
                    }
                    self.inventory.add_produce(crop);
                    self.message =
                        format!("Harvest Done! (Got {} at {:?})", crop.produce_emoji(), dir);
                    self.advance_time();
                } else {
                    self.message = String::from("Not ready yet! (Needs more time)");
                }
            } else if let Some(TileType::Mushroom) = tile {
                if self.location == Location::EastPath {
                    if let Some(map_row) = self.east_path_map.get_mut(y) {
                        map_row[x] = TileType::Grass;
                    }
                    self.inventory.add_forage(ForageType::Mushroom);
                    self.message = String::from("Harvest Done! (Got 🍄)");
                    self.advance_time();
                } else {
                    self.message = String::from("Cannot harvest mushrooms here!");
                }
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Out of bounds!");
        }
    }

    pub fn trade_action(&mut self) {
        if self.shop_state == ShopState::None {
            self.shop_state = ShopState::BuyMenu;
            self.shop_cursor = 0;
            self.message = String::from("Shop opened!");
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
        if self.home_state == HomeState::None && self.hour == 2 && self.location == Location::Farm {
            self.home_state = HomeState::Alert;
            self.home_cursor = 0;
            self.message = String::from("It's late. You should rest.");
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
                        self.message = format!("Bought 🫙 {}!", crop.seed_name());
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
                    self.message = String::from("Back to buy menu.");
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
        self.message = String::from("Good morning! Ready for another day.");
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
        assert!(TileType::House.is_walkable());
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
}
