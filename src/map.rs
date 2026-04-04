use crate::types::{Direction, FlowerState, Region, TileType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionMap {
    pub region: Region,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
}

impl RegionMap {
    pub fn new(region: Region, width: usize, height: usize) -> Self {
        let tiles = vec![vec![TileType::Grass; width]; height];
        Self {
            region,
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TileType> {
        if x < self.width && y < self.height {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: TileType) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get(x, y).map(|t| t.is_walkable()).unwrap_or(false)
    }
}

pub fn create_farm_map() -> RegionMap {
    let mut map = RegionMap::new(Region::Farm, 8, 8);

    for x in 0..8 {
        map.set(x, 0, TileType::Boundary);
        map.set(x, 7, TileType::Boundary);
    }
    for y in 0..8 {
        map.set(0, y, TileType::Boundary);
        map.set(7, y, TileType::Boundary);
    }

    map.set(2, 2, TileType::House);
    map.set(7, 5, TileType::PathEast);

    map
}

pub fn create_east_path_map() -> RegionMap {
    let mut map = RegionMap::new(Region::EastPath, 11, 4);

    for x in 0..11 {
        map.set(x, 0, TileType::Boundary);
        map.set(x, 3, TileType::Boundary);
    }
    for y in 0..4 {
        map.set(10, y, TileType::Boundary);
    }

    map.set(0, 2, TileType::PathFarm);
    map.set(5, 0, TileType::PathSquare);
    map.set(2, 3, TileType::PathSouthRiver);
    map.set(9, 2, TileType::Mushroom);

    map
}

pub fn create_square_map() -> RegionMap {
    let mut map = RegionMap::new(Region::Square, 9, 5);

    for x in 0..9 {
        map.set(x, 0, TileType::Boundary);
        map.set(x, 4, TileType::Boundary);
    }
    for y in 0..5 {
        map.set(0, y, TileType::Boundary);
        map.set(8, y, TileType::Boundary);
    }

    map.set(4, 2, TileType::Fountain);
    map.set(1, 1, TileType::Flower(FlowerState { mature: true }));
    map.set(4, 4, TileType::PathSquare);

    map
}

pub fn create_south_river_map() -> RegionMap {
    let mut map = RegionMap::new(Region::SouthRiver, 13, 4);

    for x in 0..13 {
        map.set(x, 0, TileType::Boundary);
    }
    for y in 2..4 {
        for x in 0..13 {
            map.set(x, y, TileType::River);
        }
    }

    map.set(2, 0, TileType::PathSouthRiverGate);

    map
}

pub fn create_initial_maps() -> HashMap<Region, RegionMap> {
    let mut maps = HashMap::new();
    maps.insert(Region::Farm, create_farm_map());
    maps.insert(Region::EastPath, create_east_path_map());
    maps.insert(Region::Square, create_square_map());
    maps.insert(Region::SouthRiver, create_south_river_map());
    maps
}

pub struct TransitionResult {
    pub new_region: Region,
    pub new_x: usize,
    pub new_y: usize,
    pub new_direction: Direction,
}

#[allow(dead_code)]
pub fn get_transition(
    from_region: Region,
    from_x: usize,
    from_y: usize,
    _direction: Direction,
) -> Option<TransitionResult> {
    match (from_region, from_x, from_y) {
        (Region::Farm, 7, 5) => Some(TransitionResult {
            new_region: Region::EastPath,
            new_x: 1,
            new_y: 2,
            new_direction: Direction::Right,
        }),
        (Region::EastPath, 0, 2) => Some(TransitionResult {
            new_region: Region::Farm,
            new_x: 6,
            new_y: 5,
            new_direction: Direction::Left,
        }),
        (Region::EastPath, 5, 0) => Some(TransitionResult {
            new_region: Region::Square,
            new_x: 4,
            new_y: 3,
            new_direction: Direction::Up,
        }),
        (Region::Square, 4, 4) => Some(TransitionResult {
            new_region: Region::EastPath,
            new_x: 5,
            new_y: 1,
            new_direction: Direction::Down,
        }),
        (Region::EastPath, 2, 3) => Some(TransitionResult {
            new_region: Region::SouthRiver,
            new_x: 2,
            new_y: 1,
            new_direction: Direction::Down,
        }),
        (Region::SouthRiver, 2, 0) => Some(TransitionResult {
            new_region: Region::EastPath,
            new_x: 2,
            new_y: 2,
            new_direction: Direction::Up,
        }),
        _ => None,
    }
}
