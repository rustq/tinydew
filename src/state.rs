use crate::world::{
    CropState, CropType, Direction, EAST_PATH_HEIGHT, EAST_PATH_WIDTH, FARM_HEIGHT, FARM_WIDTH,
    Map, TileType, create_east_path_map, create_farm_map,
};
use crossterm::event::KeyCode;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Farm,
    EastPath,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub seeds: HashMap<CropType, u32>,
    pub produce: HashMap<CropType, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            seeds: HashMap::new(),
            produce: HashMap::new(),
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
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopState {
    None,
    BuyMenu,
    SellMenu,
}

#[derive(Debug, Clone)]
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
    pub weather: String,
    pub inventory: Inventory,
    pub selected_seed: CropType,
    pub money: u32,
    pub shop_state: ShopState,
    pub shop_cursor: usize,
}

impl GameState {
    pub fn new() -> Self {
        let mut farm_map = create_farm_map();
        let (player_x, player_y) = find_player_start(&farm_map);

        let mut east_path_map = create_east_path_map();

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
            weather: String::from("Sunny"),
            inventory: Inventory::new(),
            selected_seed: CropType::Carrot,
            money: 500,
            shop_state: ShopState::None,
            shop_cursor: 0,
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
        let was_night = self.hour >= 20;

        self.minute += 5;
        if self.minute >= 60 {
            self.minute = 0;
            self.hour += 1;
        }
        if self.hour >= 24 {
            self.hour = 0;
            self.day += 1;
        }

        if was_night && self.hour >= 6 {
            self.start_new_day();
        }
    }

    pub fn start_new_day(&mut self) {
        self.roll_weather();

        for y in 0..FARM_HEIGHT {
            for x in 0..FARM_WIDTH {
                if let TileType::Crop(crop, state) = &mut self.farm_map[y][x] {
                    if state.watered_today {
                        state.days_grown += 1;
                    }
                    state.watered_today = false;
                }
            }
        }

        self.message = String::from("Good morning! A new day begins.");
    }

    fn roll_weather(&mut self) {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let weather_idx = ((seed + self.day as u64) % 3) as usize;
        self.weather = match weather_idx {
            0 => String::from("Sunny"),
            1 => String::from("Rain"),
            _ => String::from("Cloudy"),
        };
    }

    pub fn is_night(&self) -> bool {
        self.hour >= 20 || self.hour < 6
    }

    pub fn get_weather_icon(&self) -> &'static str {
        if self.is_night() {
            "🌙"
        } else {
            match self.weather.as_str() {
                "Sunny" => "☀️",
                "Rain" => "🌧",
                "Cloudy" => "☁️",
                _ => "☀️",
            }
        }
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
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Nothing in front!");
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
}

fn find_player_start(map: &Map) -> (usize, usize) {
    (3, 3)
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
