use crate::economy::{Inventory, Shop};
use crate::entity::Player;
use crate::fishing::reset_river_bubbles;
use crate::map::{RegionMap, create_initial_maps, get_transition};
use crate::season::get_festival_message;
use crate::spawn::{spawn_daily_flowers, spawn_daily_mushrooms};
use crate::time::GameTime;
use crate::types::{CropType, Direction, FlowerState, Region, TileType, Weather};
use crate::weather::roll_weather;
use rand::{Rng, SeedableRng};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropData {
    pub x: usize,
    pub y: usize,
    pub crop_type: CropType,
    pub days_grown: u32,
    pub watered: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CropsMap(HashMap<(Region, usize, usize), CropData>);

impl CropsMap {
    pub fn get(&self, key: &(Region, usize, usize)) -> Option<&CropData> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &(Region, usize, usize)) -> Option<&mut CropData> {
        self.0.get_mut(key)
    }

    pub fn contains_key(&self, key: &(Region, usize, usize)) -> bool {
        self.0.contains_key(key)
    }

    pub fn insert(&mut self, key: (Region, usize, usize), value: CropData) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, key: &(Region, usize, usize)) -> Option<CropData> {
        self.0.remove(key)
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, (Region, usize, usize), CropData> {
        self.0.iter_mut()
    }
}

impl Serialize for CropsMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for ((region, x, y), data) in &self.0 {
            let key = format!("{:?}-{}-{}", region, x, y);
            map.serialize_entry(&key, data)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for CropsMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, CropData>::deserialize(deserializer)?;
        let mut crops = CropsMap::default();
        for (key, data) in map {
            let parts: Vec<&str> = key.splitn(2, '-').collect();
            if parts.len() == 2 {
                let region = match parts[0] {
                    "Farm" => Region::Farm,
                    "EastPath" => Region::EastPath,
                    "Square" => Region::Square,
                    "SouthRiver" => Region::SouthRiver,
                    _ => continue,
                };
                let coords: Vec<&str> = parts[1].split('-').collect();
                if coords.len() == 2 {
                    if let (Ok(x), Ok(y)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>())
                    {
                        crops.0.insert((region, x, y), data);
                    }
                }
            }
        }
        Ok(crops)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub time: GameTime,
    pub weather: Weather,
    pub player: Player,
    pub money: u32,
    pub inventory: Inventory,
    pub maps: HashMap<Region, RegionMap>,
    pub crops: CropsMap,
    pub bottom_message: String,
    pub is_new_game: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let maps = create_initial_maps();
        let weather = Weather::Sunny;
        let time = GameTime::start_of_day(1);

        let player = Player::new(6, 5, Region::Farm);

        let mut state = Self {
            time,
            weather,
            player,
            money: 100,
            inventory: Inventory::new(),
            maps,
            crops: CropsMap::default(),
            bottom_message: "Welcome to TinyDew!".to_string(),
            is_new_game: true,
        };

        state.inventory.add_seeds(5);
        state
    }

    pub fn get_tile(&self, region: Region, x: usize, y: usize) -> Option<TileType> {
        if let Some(crop_data) = self.crops.get(&(region, x, y)) {
            return Some(TileType::Crop(crop_data.crop_type, crop_data.is_mature()));
        }
        self.maps.get(&region).and_then(|m| m.get(x, y))
    }

    pub fn get_current_map(&self) -> Option<&RegionMap> {
        self.maps.get(&self.player.region)
    }

    pub fn get_current_map_mut(&mut self) -> Option<&mut RegionMap> {
        self.maps.get_mut(&self.player.region)
    }

    pub fn process_day_transition(&mut self) {
        let new_day = self.time.day + 1;
        self.time.day = new_day;
        self.time.minutes = 6 * 60;

        self.weather = roll_weather(new_day, new_day as u64);

        for (_, crop) in self.crops.iter_mut() {
            if crop.watered {
                crop.days_grown += 1;
            }
            crop.watered = false;
        }

        reset_river_bubbles(&mut self.maps);

        let seed = (new_day as u64).wrapping_mul(12345);
        spawn_daily_flowers(&mut self.maps, new_day, seed);
        spawn_daily_mushrooms(&mut self.maps, new_day, seed);

        crate::festival::update_festival_state(&mut self.maps, new_day);

        if let Some(msg) = get_festival_message(new_day) {
            self.bottom_message = msg;
        } else {
            self.bottom_message = format!("Good morning! Day {}.", new_day);
        }
    }

    pub fn try_move(&mut self, direction: Direction) -> String {
        let (dx, dy) = direction.delta();
        let new_x = self.player.x as i32 + dx;
        let new_y = self.player.y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return "You can't go that way.".to_string();
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if let Some(map) = self.get_current_map() {
            if !map.is_walkable(new_x, new_y) {
                if let Some(tile) = map.get(new_x, new_y) {
                    if matches!(tile, TileType::Crop(_, true))
                        || matches!(tile, TileType::Flower(FlowerState { mature: true }))
                    {
                        return "There's a mature crop here. Harvest it first.".to_string();
                    }
                    if matches!(tile, TileType::Wonder) {
                        self.player.x = new_x;
                        self.player.y = new_y;
                        self.player.direction = direction;
                        self.advance_time(5);
                        return crate::festival::handle_wonder_message().to_string();
                    }
                }
                return "You can't walk there.".to_string();
            }
        }

        self.player.x = new_x;
        self.player.y = new_y;
        self.player.direction = direction;
        self.advance_time(5);

        if let Some(transition) =
            get_transition(self.player.region, self.player.x, self.player.y, direction)
        {
            self.player.region = transition.new_region;
            self.player.x = transition.new_x;
            self.player.y = transition.new_y;
            self.player.direction = transition.new_direction;
            self.bottom_message = format!("Entered {}.", self.player.region);
            return self.bottom_message.clone();
        }

        self.bottom_message = "You move.".to_string();
        self.bottom_message.clone()
    }

    pub fn try_water(&mut self, direction: Direction) -> String {
        let (dx, dy) = direction.delta();
        let x = self.player.x as i32 + dx;
        let y = self.player.y as i32 + dy;

        if x < 0 || y < 0 {
            return "Nothing to water there.".to_string();
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(crop_data) = self.crops.get_mut(&(self.player.region, x, y)) {
            crop_data.watered = true;
            self.advance_time(5);
            self.bottom_message = "Watered the crop.".to_string();
            return self.bottom_message.clone();
        }

        "Nothing to water there.".to_string()
    }

    pub fn try_plant(&mut self, direction: Direction) -> String {
        if self.player.region != Region::Farm {
            return "You can't plant here.".to_string();
        }

        if !self.inventory.has_seeds() {
            return "No seeds available.".to_string();
        }

        let (dx, dy) = direction.delta();
        let x = self.player.x as i32 + dx;
        let y = self.player.y as i32 + dy;

        if x < 0 || y < 0 {
            return "Can't plant there.".to_string();
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(map) = self.get_current_map() {
            if let Some(tile) = map.get(x, y) {
                match tile {
                    TileType::Grass | TileType::Soil => {
                        if self.crops.contains_key(&(self.player.region, x, y)) {
                            return "Something is already planted here.".to_string();
                        }

                        self.inventory.use_seed();

                        let mut rng = rand::rngs::StdRng::from_entropy();
                        let roll = rng.gen_range(0..4);
                        let crop_type = match roll {
                            0 => CropType::Carrot,
                            1 => CropType::Strawberry,
                            2 => CropType::Cauliflower,
                            _ => CropType::Flower,
                        };

                        if let Some(map_mut) = self.get_current_map_mut() {
                            map_mut.set(x, y, TileType::Soil);
                        }

                        self.crops.insert(
                            (self.player.region, x, y),
                            CropData {
                                x,
                                y,
                                crop_type,
                                days_grown: 0,
                                watered: false,
                            },
                        );

                        self.advance_time(5);
                        self.bottom_message = format!("Planted a {}!", crop_type.produce_emoji());
                        return self.bottom_message.clone();
                    }
                    _ => return "Can't plant here.".to_string(),
                }
            }
        }

        "Can't plant here.".to_string()
    }

    pub fn try_harvest(&mut self, direction: Direction) -> String {
        let (dx, dy) = direction.delta();
        let x = self.player.x as i32 + dx;
        let y = self.player.y as i32 + dy;

        if x < 0 || y < 0 {
            return "Nothing to harvest.".to_string();
        }

        let x = x as usize;
        let y = y as usize;

        if self.player.region == Region::Square {
            if let Some(map) = self.get_current_map() {
                if let Some(TileType::Flower(FlowerState { mature: true })) = map.get(x, y) {
                    if let Some(map_mut) = self.get_current_map_mut() {
                        map_mut.set(x, y, TileType::Grass);
                    }
                    self.inventory.add_produce("🌺", 1);
                    self.advance_time(5);
                    self.bottom_message = "Harvested a flower!".to_string();
                    return self.bottom_message.clone();
                }
            }
        }

        if self.player.region == Region::EastPath {
            if let Some(map) = self.get_current_map() {
                if let Some(TileType::Mushroom) = map.get(x, y) {
                    if let Some(map_mut) = self.get_current_map_mut() {
                        map_mut.set(x, y, TileType::Grass);
                    }
                    self.inventory.add_forage("🍄", 1);
                    self.advance_time(5);
                    self.bottom_message = "Harvested a mushroom!".to_string();
                    return self.bottom_message.clone();
                }
            }
        }

        if let Some(crop_data) = self.crops.get(&(self.player.region, x, y)) {
            if crop_data.is_mature() {
                let _crop_type = crop_data.crop_type;
                let emoji = crop_data.crop_type.produce_emoji().to_string();

                self.crops.remove(&(self.player.region, x, y));

                if let Some(map) = self.get_current_map() {
                    if matches!(map.get(x, y), Some(TileType::Crop(_, _))) {
                        if let Some(map_mut) = self.get_current_map_mut() {
                            map_mut.set(x, y, TileType::Grass);
                        }
                    }
                }

                self.inventory.add_produce(&emoji, 1);
                self.advance_time(5);
                self.bottom_message = format!("Harvested {}!", emoji);
                return self.bottom_message.clone();
            } else {
                return "This crop isn't ready yet.".to_string();
            }
        }

        "Nothing to harvest.".to_string()
    }

    pub fn try_clear(&mut self, direction: Direction) -> String {
        if self.player.region == Region::Square {
            return "You can't clear here.".to_string();
        }

        if self.player.region == Region::EastPath {
            return "You can't clear here.".to_string();
        }

        let (dx, dy) = direction.delta();
        let x = self.player.x as i32 + dx;
        let y = self.player.y as i32 + dy;

        if x < 0 || y < 0 {
            return "Can't clear there.".to_string();
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(map) = self.get_current_map() {
            if let Some(tile) = map.get(x, y) {
                if self.crops.contains_key(&(self.player.region, x, y)) {
                    return "There's a crop here. Harvest it first.".to_string();
                }

                match tile {
                    TileType::Grass => {
                        if let Some(map_mut) = self.get_current_map_mut() {
                            map_mut.set(x, y, TileType::Soil);
                        }
                        self.advance_time(5);
                        self.bottom_message = "Cleared the tile.".to_string();
                        return self.bottom_message.clone();
                    }
                    TileType::Soil => {
                        if let Some(map_mut) = self.get_current_map_mut() {
                            map_mut.set(x, y, TileType::Grass);
                        }
                        self.advance_time(5);
                        self.bottom_message = "Cleared the tile.".to_string();
                        return self.bottom_message.clone();
                    }
                    _ => return "Can't clear this tile.".to_string(),
                }
            }
        }

        "Can't clear this tile.".to_string()
    }

    pub fn try_fish(&mut self, direction: Direction) -> String {
        if self.player.region != Region::SouthRiver {
            return "You can only fish in the river.".to_string();
        }

        let (dx, dy) = direction.delta();
        let x = self.player.x as i32 + dx;
        let y = self.player.y as i32 + dy;

        if x < 0 || y < 0 {
            return "You can't fish there.".to_string();
        }

        let x = x as usize;
        let y = y as usize;

        let seed = self.time.day as u64 * 1000 + self.time.minutes as u64;
        let result = crate::fishing::try_fish(self.get_current_map_mut().unwrap(), x, y, seed);

        if let Some(fish) = result.caught {
            self.inventory.add_fish(fish, 1);
        }

        self.advance_time(10);
        self.bottom_message = result.message;
        self.bottom_message.clone()
    }

    pub fn buy_seed(&mut self) -> String {
        let shop = Shop::new();
        if self.money >= shop.seed_price {
            self.money -= shop.seed_price;
            self.inventory.add_seeds(1);
            self.bottom_message = "Bought a seed.".to_string();
            self.bottom_message.clone()
        } else {
            "Not enough money.".to_string()
        }
    }

    pub fn sell_item(&mut self, emoji: &str) -> String {
        let shop = Shop::new();

        if let Some(price) = shop.produce_prices.get(emoji) {
            if self.inventory.remove_produce(emoji, 1) {
                self.money += price;
                self.bottom_message = format!("Sold {} for ${}!", emoji, price);
                return self.bottom_message.clone();
            }
            return "You don't have any of those.".to_string();
        }

        if let Some(price) = shop.fish_prices.get(emoji) {
            if self.inventory.remove_fish(emoji, 1) {
                self.money += price;
                self.bottom_message = format!("Sold {} for ${}!", emoji, price);
                return self.bottom_message.clone();
            }
            return "You don't have any of those.".to_string();
        }

        if emoji == "🍄" {
            if self.inventory.remove_forage("🍄", 1) {
                self.money += shop.mushroom_price;
                self.bottom_message = format!("Sold 🍄 for ${}!", shop.mushroom_price);
                return self.bottom_message.clone();
            }
            return "You don't have any mushrooms.".to_string();
        }

        "You can't sell that.".to_string()
    }

    pub fn sleep(&mut self) -> String {
        if let Some(map) = self.get_current_map() {
            if !matches!(map.get(self.player.x, self.player.y), Some(TileType::House))
                && self.player.region != Region::Farm
            {
                return "You can only sleep in your house.".to_string();
            }
        }

        self.process_day_transition();

        self.player.region = Region::Farm;
        self.player.x = 3;
        self.player.y = 3;

        self.bottom_message = format!("Good morning! Day {}.", self.time.day);
        self.bottom_message.clone()
    }

    pub fn advance_time(&mut self, minutes: u32) {
        self.time.advance(minutes);
    }

    pub fn get_greeting(&self) -> String {
        if let Some(msg) = get_festival_message(self.time.day) {
            return msg;
        }

        let hour = self.time.hour();
        let weather_msg = match self.weather {
            Weather::Sunny => "The sun is shining.",
            Weather::Cloudy => "It's a cloudy day.",
            Weather::Rainy => "It's raining.",
        };

        if self.time.is_night() {
            if hour >= 20 {
                "Night has fallen. Try sleeping until morning.".to_string()
            } else {
                "It's getting late. You should rest soon.".to_string()
            }
        } else if hour < 12 {
            format!("Good morning! {}", weather_msg)
        } else if hour < 17 {
            format!("Good afternoon! {}", weather_msg)
        } else {
            format!("Good evening! {}", weather_msg)
        }
    }
}

impl CropData {
    pub fn is_mature(&self) -> bool {
        self.days_grown >= self.crop_type.maturity_days()
    }
}

pub struct GameDatabase {
    conn: Connection,
}

impl GameDatabase {
    pub fn new(path: &PathBuf) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             CREATE TABLE IF NOT EXISTS schema_version (
                 id INTEGER PRIMARY KEY CHECK (id = 1),
                 version INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS game_save (
                 id INTEGER PRIMARY KEY CHECK (id = 1),
                 updated_at TEXT NOT NULL,
                 payload TEXT NOT NULL
             )",
        )?;

        let version: i64 = conn
            .query_row(
                "SELECT version FROM schema_version WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(1);

        if version < 1 {
            conn.execute(
                "INSERT OR REPLACE INTO schema_version (id, version) VALUES (1, 1)",
                [],
            )?;
        }

        Ok(Self { conn })
    }

    pub fn load(&self) -> Option<GameState> {
        let payload: String = self
            .conn
            .query_row("SELECT payload FROM game_save WHERE id = 1", [], |row| {
                row.get(0)
            })
            .ok()?;

        serde_json::from_str(&payload).ok()
    }

    pub fn save(&self, state: &GameState) -> Result<(), rusqlite::Error> {
        let payload = serde_json::to_string(state).unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO game_save (id, updated_at, payload) VALUES (1, ?1, ?2)",
            params![now, payload],
        )?;

        Ok(())
    }
}

fn get_db_path() -> PathBuf {
    if let Ok(env_path) = std::env::var("TINYDEW_DB_PATH") {
        return PathBuf::from(env_path);
    }

    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("tinydew").join("tinydew.sqlite")
}

static DATABASE: std::sync::OnceLock<std::sync::Mutex<GameDatabase>> = std::sync::OnceLock::new();

fn get_database() -> &'static std::sync::Mutex<GameDatabase> {
    DATABASE.get_or_init(|| {
        let path = get_db_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::sync::Mutex::new(GameDatabase::new(&path).expect("Failed to open database"))
    })
}

pub fn load_game() -> GameState {
    get_database().lock().unwrap().load().unwrap_or_default()
}

pub fn save_game(state: &GameState) {
    get_database()
        .lock()
        .unwrap()
        .save(state)
        .expect("Failed to save game");
}
