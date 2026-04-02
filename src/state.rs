use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::economy::Inventory;
use crate::entity::{Direction, Entity};
use crate::map::{self, Region, RegionMap};
use crate::time::WorldTime;
use crate::weather::Weather;

const SAVE_FILE: &str = "tinydew_save.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub day: u32,
    pub time: WorldTime,
    pub weather: Weather,
    pub player: Entity,
    pub guest: Entity,
    pub inventory: Inventory,
    pub maps: HashMap<String, RegionMap>,
    pub message: String,
}

impl Default for GameState {
    fn default() -> Self {
        let mut maps = HashMap::new();
        maps.insert("Farm".to_string(), map::create_farm());
        maps.insert("EastPath".to_string(), map::create_east_path());
        maps.insert("Square".to_string(), map::create_square());
        maps.insert("SouthRiver".to_string(), map::create_south_river());

        Self {
            day: 1,
            time: WorldTime::new(6, 0),
            weather: Weather::Sunny,
            player: Entity::new(3, 3, Region::Farm, Direction::Down),
            guest: Entity::new(5, 3, Region::Farm, Direction::Down),
            inventory: Inventory::default(),
            maps,
            message: "Welcome to TinyDew!".to_string(),
        }
    }
}

impl GameState {
    pub fn region_key(region: Region) -> &'static str {
        match region {
            Region::Farm => "Farm",
            Region::EastPath => "EastPath",
            Region::Square => "Square",
            Region::SouthRiver => "SouthRiver",
        }
    }

    pub fn get_map(&self, region: Region) -> &RegionMap {
        self.maps.get(Self::region_key(region)).unwrap()
    }

    pub fn get_map_mut(&mut self, region: Region) -> &mut RegionMap {
        self.maps.get_mut(Self::region_key(region)).unwrap()
    }

    pub fn tick_time(&mut self) {
        self.time.tick();
    }

    pub fn load() -> Self {
        let path = Path::new(SAVE_FILE);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(data) => match serde_json::from_str(&data) {
                    Ok(state) => return state,
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SAVE_FILE, data);
        }
    }
}
