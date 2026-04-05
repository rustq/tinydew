use serde::{Deserialize, Serialize};

use crate::map;
use crate::types::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub day: u32,
    pub time_minutes: u32,
    pub weather: Weather,
    pub season: Season,
    pub player: Player,
    pub maps: RegionMaps,
    pub inventory: Inventory,
    pub money: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            day: 1,
            time_minutes: 360, // 06:00
            weather: Weather::Sunny,
            season: Season::Spring,
            player: Player {
                x: 3,
                y: 3,
                region: Region::Farm,
                direction: Direction::Down,
            },
            maps: RegionMaps {
                farm: map::create_farm(),
                east_path: map::create_east_path(),
                square: map::create_square(),
                south_river: map::create_south_river(),
            },
            inventory: Inventory {
                seeds: 1,
                ..Inventory::default()
            },
            money: 100,
        }
    }
}
